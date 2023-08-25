use std::fmt;
use rand::seq::SliceRandom;
use std::collections::HashSet;

#[derive(PartialEq, Clone)]
pub struct SudokuGrid {
    pub grid: [[u8; 9]; 9],
    pub candidates: [[HashSet<u8>; 9]; 9]
}

impl fmt::Display for SudokuGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..9 {
            for sub_row in 0..3 {
                for col in 0..9 {
                    if sub_row == 1 {
                        // Write the grid value
                        write!(f, " {} ", self.grid[row][col])?;
                    } else {
                        // Write an empty placeholder for the grid value
                        write!(f, " . ")?;
                    }
                    
                    for i in (sub_row * 3 + 1)..=(sub_row * 3 + 3) {
                        if self.candidates[row][col].contains(&i) {
                            write!(f, "{}", i)?;
                        } else {
                            write!(f, ".")?;
                        }
                    }
                    
                    if col % 3 == 2 && col != 8 {
                        write!(f, " |  ")?;
                    } else {
                        write!(f, "   ")?; // Space between individual cells
                    }
                }
                writeln!(f)?;
            }
            if row % 3 == 2 && row != 8 {
                writeln!(f, "-----------------------+-----------------------+-----------------------")?;
            }
            writeln!(f)?; // Extra newline after the entire group of cell data
        }
        Ok(())
    }
}




impl SudokuGrid {
    pub fn new() -> Self {
        let initial_grid = [[0; 9]; 9];
        let all_nums: HashSet<u8> = (1..=9).collect();
        let mut initial_candidates: [[HashSet<u8>; 9]; 9] = Default::default();

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

        let mut grid = [[0 as u8; 9]; 9];
        let candidates: [[HashSet<u8>; 9]; 9] = Default::default();

        for (i, ch) in init_str.chars().enumerate() {
            let row = i / 9;
            let col = i % 9;
            match ch.to_digit(10) {
                Some(val) => grid[row][col] = val as u8,
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

    pub fn add_digit(&mut self, digit: u8, row: usize, col: usize) -> bool {
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
        let mut nums: Vec<u8> = (1..=9 as u8).collect();
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

    fn order_domain_values(&self, row: usize, col: usize) -> Vec<u8> {
        let cells_affects = |val: u8| -> i32 {
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
        let mut ret : Vec<u8> = self.candidates[row][col].clone().into_iter().collect();
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

    pub fn is_valid_sudoku_placement(&self, digit: u8, row: usize, col: usize) -> bool {
        return self.candidates[row][col].contains(&digit)
    }

    pub fn regenerate_candidates(&mut self) {
        for row in 0..9 {
            for col in 0..9 {
                if self.grid[row][col] == 0 {
                    // Start with all numbers as potential candidates
                    let mut cell_candidates: HashSet<u8> = (1..=9).collect();

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

    pub fn has_unique_solution(&mut self) -> bool {
        let mut solutions = 0;
        self.check_solutions(&mut solutions);
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

}