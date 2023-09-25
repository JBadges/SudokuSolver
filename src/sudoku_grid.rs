use std::fmt;
use itertools::{iproduct, Itertools};
use rand::seq::SliceRandom;
use std::collections::HashSet;
use super::adjacency_graph::AdjacencyGraph;

#[derive(PartialEq, Clone)]
pub struct SudokuGrid {
    pub grid: [[usize; 9]; 9],
    pub candidates: [[HashSet<usize>; 9]; 9]
}
#[derive(PartialEq, Copy, Clone)]
pub enum UnitType {
    Row,
    Col,
    Box
}

impl SudokuGrid {
    fn format_cell(&self, row: usize, col: usize, sub_row: usize) -> String {
        if self.grid[row][col] != 0 {
            // If the grid value is set, display it in the center
            if sub_row == 1 {
                return format!("  {}  ", self.grid[row][col]);
            } else {
                return "     ".to_string();
            }
        } else {
            // If the grid value is not set, display the candidates
            let start = sub_row * 3 + 1;
            let end = sub_row * 3 + 3;
            let mut s = String::new();
            for i in start..=end {
                if self.candidates[row][col].contains(&(i as usize)) {
                    s.push_str(&i.to_string());
                } else {
                    s.push('.');
                }
            }
            return format!("{:<5}", s); // Left-align the candidates to ensure consistent width
        }
    }
}

impl fmt::Display for SudokuGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Print column numbers at the top
        writeln!(f, "    0     1     2   |  3     4     5  |  6     7     8  ")?;
        writeln!(f, "--------------------+-----------------+----------------")?;

        for row in 0..9 {
            for sub_row in 0..3 {
                // Print row number at the start of each new cell row
                if sub_row == 1 {
                    write!(f, "{} |", row)?;
                } else {
                    write!(f, "  |")?;
                }

                for col in 0..9 {
                    write!(f, "{}", self.format_cell(row, col, sub_row))?;
                    if col % 3 == 2 && col != 8 {
                        write!(f, "|")?;
                    } else {
                        write!(f, " ")?; // Space between individual cells
                    }
                }
                writeln!(f)?;
            }
            if row % 3 == 2 && row != 8 {
                writeln!(f, "--------------------+-----------------+----------------")?;
            }
        }
        Ok(())
    }
}


impl SudokuGrid {
    pub fn new() -> Self {
        let initial_grid = [[0; 9]; 9];
        let all_nums: HashSet<usize> = (1..=9).collect();
        let mut initial_candidates: [[HashSet<usize>; 9]; 9] = Default::default();

        for i in 0..9 {
            for j in 0..9 {
                initial_candidates[i][j] = all_nums.clone();
            }
        }

        Self {
            grid: initial_grid,
            candidates: initial_candidates,
        }
    }

    pub fn from_string(init_str: &str) -> Self {
        // Ensure the string has the correct length
        assert_eq!(init_str.len(), 81, "Input string must have exactly 81 characters.");

        let mut grid = [[0 as usize; 9]; 9];
        let candidates: [[HashSet<usize>; 9]; 9] = Default::default();

        for (i, ch) in init_str.chars().enumerate() {
            let row = i / 9;
            let col = i % 9;
            match ch.to_digit(10) {
                Some(val) => grid[row][col] = val as usize,
                None => panic!("Invalid character in input string."),
            };
        }

        let mut ret = Self {
            grid,
            candidates,
        };
        ret.regenerate_candidates();
        ret
    }

    pub fn add_digit(&mut self, digit: usize, row: usize, col: usize) -> bool {
        let temp_candidates = self.candidates.clone();

        let mut update_candidates = || -> bool {
            if !self.is_valid_sudoku_placement(digit, row, col) {
                return false;
            }
            // Row/Col checks
            for i in 0..9 {
                if i != col {
                    self.candidates[row][i].remove(&digit);
                    if self.candidates[row][i].is_empty() { return false; }
                }
                if i != row {
                    self.candidates[i][col].remove(&digit);
                    if self.candidates[i][col].is_empty() { return false; }
                }
            }

            // Box checks
            let start_row = 3 * (row / 3);
            let start_col = 3 * (col / 3);
            for i in 0..3 {
                for j in 0..3 {
                    if i + start_row != row && j + start_col != col {
                        self.candidates[i + start_row][j + start_col].remove(&digit);
                        if self.candidates[i + start_row][j + start_col].is_empty() { return false; }
                    }
                }
            }
            self.grid[row][col] = digit;
            self.candidates[row][col].clear();
            self.candidates[row][col].insert(digit);
            true
        };
        let able_to_add_digit = update_candidates();
        if !able_to_add_digit {
            self.candidates = temp_candidates;
        }
        return able_to_add_digit;
    }

    pub fn create_sudoku_grid_randomly() -> SudokuGrid {
        let mut sgrid = SudokuGrid::new();
        // Latin initialization
        // Generate random numbers for the three main box diagonals and backtrack to fill the rest
        let mut nums: Vec<usize> = (1..=9 as usize).collect();
        for i in (0..9).step_by(3) {
            nums.shuffle(&mut rand::thread_rng()); // You'll need the `rand` crate for shuffling.
            for j in 0..3 {
                for k in 0..3 {
                    sgrid.add_digit(nums[j * 3 + k], i + j, i + k);
                }
            }
        }

        assert!(sgrid.backtrack_fill(), "Unable to solve latin init");

        return sgrid;
    }

    pub fn create_sudoku_puzzle(removals: usize) -> SudokuGrid {
        let mut sgrid = SudokuGrid::create_sudoku_grid_randomly();
        let mut iterations = removals;
        while iterations > 0 {
            let mut cells: Vec<(usize, usize)> = Vec::new();
            for row in 0..9 {
                for col in 0..9 {
                    if sgrid.grid[row][col] != 0 { // Assuming 0 means an unfilled cell
                        cells.push((row, col));
                    }
                }
            }
            cells.shuffle(&mut rand::thread_rng());
            let (row, col) = cells[0];
            let old_val = sgrid.grid[row][col];
            let old_candidates = sgrid.candidates.clone();

            sgrid.grid[row][col] = 0;
            sgrid.regenerate_candidates();
            if !sgrid.has_unique_solution() {
                sgrid.grid[row][col] = old_val;
                sgrid.candidates = old_candidates;
            }
            iterations -= 1;
        }
        return sgrid;
    }

    fn order_unassigned_variables(&self) -> Vec<(usize, usize)> {
        // Most Constrained Variable
        // TODO: most_constraining
        let mut cells: Vec<(usize, usize)> = Vec::new();
        for row in 0..9 {
            for col in 0..9 {
                if self.grid[row][col] == 0 { // Assuming 0 means an unfilled cell
                    cells.push((row, col));
                }
            }
        }
        cells.sort_by_key(|&(row, col)| self.candidates[row][col].len());
        return cells;
    }

    fn order_domain_values(&self, row: usize, col: usize) -> Vec<usize> {
        let cells_affects = |val: usize| -> i32 {
            let mut cells = 0;
            for i in 0..9 {
                if self.candidates[row][i].contains(&val) {
                    cells += 1;
                }
                if self.candidates[i][col].contains(&val) {
                    cells += 1;
                }
            }

            let box_tl_row = (row / 3) * 3;
            let box_tl_col = (col / 3) * 3;
            for i in box_tl_row..box_tl_row+3 {
                for j in box_tl_col..box_tl_col+3 {
                    if self.candidates[i][j].contains(&val) {
                        cells += 1;
                    }
                }
            }
            
            return cells - 3 // Will count itself 3 times
        };
        let mut ret : Vec<usize> = self.candidates[row][col].clone().into_iter().collect();
        // Least constraining value, pick value that limits the least
        ret.sort_by_key(|x| cells_affects(*x));
        return ret;
    }

    pub fn backtrack_fill(&mut self) -> bool {
        let unassigned = self.order_unassigned_variables();
        if unassigned.is_empty() {
            return true;
        }

        let (row, col) = unassigned[0];

        for digit in self.order_domain_values(row, col) {
            let temp_cands = self.candidates.clone();
            if self.add_digit(digit, row, col) {
                if self.backtrack_fill() {
                    return true;
                }
            }
            self.grid[row][col] = 0;
            self.candidates = temp_cands;
        }

        return false;
    }

    pub fn is_valid_sudoku_placement(&self, digit: usize, row: usize, col: usize) -> bool {
        return self.candidates[row][col].contains(&digit)
    }

    pub fn regenerate_candidates(&mut self) {
        for row in 0..9 {
            for col in 0..9 {
                if self.grid[row][col] == 0 {
                    // Start with all numbers as potential candidates
                    let mut cell_candidates: HashSet<usize> = (1..=9).collect();

                    // Remove numbers from the same row, column, and box
                    for i in 0..9 {
                        cell_candidates.remove(&self.grid[row][i]);
                        cell_candidates.remove(&self.grid[i][col]);
                    }

                    let start_row = 3 * (row / 3);
                    let start_col = 3 * (col / 3);
                    for i in 0..3 {
                        for j in 0..3 {
                            cell_candidates.remove(&self.grid[start_row + i][start_col + j]);
                        }
                    }

                    self.candidates[row][col] = cell_candidates;
                } else {
                    // If there's a number in the cell, the set of candidates should be the digit
                    self.candidates[row][col].clear();
                    self.candidates[row][col].insert(self.grid[row][col]);
                }
            }
        }
    }

    pub fn to_number_string(&self) -> String {
        self.grid.iter()
            .flatten()
            .map(|&num| num.to_string())
            .collect::<Vec<_>>()
            .join("")
    }

    pub fn has_unique_solution(&self) -> bool {
        let mut solutions = 0;
        let mut a = self.clone();
        a.check_solutions(&mut solutions);
        solutions == 1
    }

    fn check_solutions(&mut self, solutions: &mut u32) {
        // If already found multiple solutions, exit early
        if *solutions > 1 {
            return;
        }

        let unassigned = self.order_unassigned_variables();

        // If no unassigned variables are left, we found a solution
        if unassigned.is_empty() {
            *solutions += 1;
            return;
        }

        let (row, col) = unassigned[0];
        for &digit in &self.order_domain_values(row, col) {
            if self.is_valid_sudoku_placement(digit, row, col) {
                // Save current state to revert back after recursion
                let old_val = self.grid[row][col];
                let old_candidates = self.candidates.clone();

                if self.add_digit(digit, row, col) {
                    self.check_solutions(solutions);

                    // Revert changes to check next candidate
                    self.grid[row][col] = old_val;
                    self.candidates = old_candidates;
                }
            }
        }
    }

    pub fn generate_cells_seen_from_cord(cord: (usize, usize)) -> HashSet<(usize, usize)> {
        let (row, col) = cord;
        let mut cells = HashSet::new();
        for i in 0..9 {
            cells.insert((i, col));
            cells.insert((row, i));
        }
        for box_row in 3*(row/3)..3*(row/3)+3 {
            for box_col in 3*(col/3)..3*(col/3)+3 {
                cells.insert((box_row, box_col));
            }
        }
        cells
    }

    pub fn cells_see_each_other(corda: (usize, usize), cordb: (usize, usize)) -> bool {
        corda.0 == cordb.0 || corda.1 == cordb.1 || (corda.0 / 3 == cordb.0 / 3 && corda.1 / 3 == cordb.1/3)
    }

    pub fn get_conjugate_pairs(&self, num: usize) -> AdjacencyGraph {
        let mut graph = AdjacencyGraph::new();

        // Box
        for box_row in (0..9).step_by(3) {
            for box_col in (0..9).step_by(3) {
                let mut cords = Vec::new();
                for row in box_row..box_row+3 {
                    for col in box_col..box_col+3 {
                        if self.candidates[row][col].contains(&num) {
                            cords.push((row, col, num));
                        }
                    }
                }
                if cords.len() == 2 {
                    graph.add_edge(cords[0], cords[1]);
                }
            }
        }

        // Col
        for col in 0..9 {
            let mut cords = Vec::new();
            for row in 0..9 {
                if self.candidates[row][col].contains(&num) {
                    cords.push((row, col, num));
                }
            }
            if cords.len() == 2 {
                graph.add_edge(cords[0], cords[1]);
            }
        }
        // Row
        for row in 0..9 {
            let mut cords = Vec::new();
            for col in 0..9 {
                if self.candidates[row][col].contains(&num) {
                    cords.push((row, col, num));
                }
            }
            if cords.len() == 2 {
                graph.add_edge(cords[0], cords[1]);
            }
        }

        return graph;
    }

    pub fn get_contained_units(cells: &Vec<(usize, usize)>) -> Vec<UnitType> {
        if cells.is_empty() { return Default::default();}
        let mut ret = Vec::default();

        // Check if all same box
        let box_match = cells.iter().all(|(row, col)| *row/3 == cells[0].0/3 && *col/3 == cells[0].1/3);
        if box_match {
            ret.push(UnitType::Box);
        }

        // Check if all same row
        let row_match = cells.iter().all(|(row, _)| *row == cells[0].0);
        if row_match {
            ret.push(UnitType::Row);
        }
        // Check if all same col
        let col_match = cells.iter().all(|(_, col)| *col == cells[0].1);
        if col_match {
            ret.push(UnitType::Col);
        }
        
        ret
    }

    pub fn get_all_units_from_unit_type(axis: UnitType) -> Vec<Vec<(usize, usize)>> {
        match axis {
            UnitType::Box => {
                // Generate cells for boxes
                let mut boxes = Vec::new();
                for box_row in 0..3 {
                    for box_col in 0..3 {
                        let mut box_cells = Vec::new();
                        for row in 0..3 {
                            for col in 0..3 {
                                box_cells.push((box_row * 3 + row, box_col * 3 + col));
                            }
                        }
                        boxes.push(box_cells);
                    }
                }
                boxes
            },
            UnitType::Row => {
                // Generate cells for rows
                let mut rows = Vec::new();
                for row in 0..9 {
                    let mut row_cells = Vec::new();
                    for col in 0..9 {
                        row_cells.push((row, col));
                    }
                    rows.push(row_cells);
                }
                rows
            },
            UnitType::Col => {
                // Generate cells for columns
                let mut cols = Vec::new();
                for col in 0..9 {
                    let mut col_cells = Vec::new();
                    for row in 0..9 {
                        col_cells.push((row, col));
                    }
                    cols.push(col_cells);
                }
                cols
            },
        }
    }

    pub fn get_cells_in_unit_from(unit: UnitType, cell: (usize, usize)) -> Vec<(usize, usize)>{
        match unit {
            UnitType::Box => {
                // Generate cells for boxes
                let mut box_cells = Vec::new();
                for row in 0..3 {
                    for col in 0..3 {
                        box_cells.push((cell.0 / 3 * 3 + row, cell.1 / 3 * 3 + col));
                    }
                }
                box_cells
            },
            UnitType::Row => {
                // Generate cells for rows
                let mut row_cells = Vec::new();
                for col in 0..9 {
                    row_cells.push((cell.0, col));
                }
                row_cells
            },
            UnitType::Col => {
                let mut col_cells = Vec::new();
                for row in 0..9 {
                    col_cells.push((row, cell.1));
                }
                col_cells
            },
        }
    }

    // Boxes in 
    // 1 2 3
    // 4 5 6
    // 7 8 9
    pub fn get_cells_in_box_n(n: usize) -> Vec<(usize, usize)> {
        let n_index = n - 1;
        let (bx, by) = ((n_index / 3) * 3, (n_index % 3) * 3);
        iproduct!(bx..bx+3,by..by+3).collect_vec()
    }

    pub fn get_box_number_from_cell(cell: (usize, usize)) -> usize {
        let (row, col) = cell;
        (row / 3) * 3 + (col / 3) + 1
    }

    pub fn cell_to_str(cell: (usize, usize)) -> String {
        let (row, col) = cell;
        let row_char = (b'A' + row as u8) as char;
        format!("{},{}", row_char, col + 1)
    }
    
    pub fn cell_candidate_to_str(cell: (usize, usize, usize)) -> String {
        let (row, col, candidate) = cell;
        format!("{},{}", SudokuGrid::cell_to_str((row, col)), candidate)
    }

}