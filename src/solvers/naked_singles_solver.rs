use super::sudoku_solver::*;
use super::super::sudoku_grid::*;

pub struct NakedSinglesSolver;

impl SudokuSolveMethod for NakedSinglesSolver {
    fn apply(&self, sgrid: &mut SudokuGrid) -> bool {
        let mut applied = false;
        // Check for hidden singles in rows and columns
        for i in 0..9 {
            for j in 0..9 {
                if sgrid.grid[i][j] == 0 && sgrid.candidates[i][j].len() == 1 {
                    if let Some(digit) = sgrid.candidates[i][j].iter().next() {
                        println!("Solver [NakedSinglesSolver] found solution {} at ({},{})", digit, i, j);
                        assert!(sgrid.add_digit(*digit, i, j));
                        applied = true
                    }
                }
            }
        }

        fn find_last_remaining(sgrid: &mut SudokuGrid, vals: Vec<(usize, usize)>) -> bool {
            let mut applied = false;
            let mut count: Vec<i32> = vec![0; 10];
            let mut last_pos = vec![(0, 0); 10];
            
            for (row, col) in vals {
                if sgrid.grid[row][col] == 0 {
                    for &candidate in &sgrid.candidates[row][col] {
                        count[candidate as usize] += 1;
                        last_pos[candidate as usize] = (row, col);
                    }
                }
            }
    
            for num in 1..=9 {
                if count[num] == 1 {
                    let (row, col) = last_pos[num];
                    println!("Solver [NakedSinglesSolver] found solution {} at ({},{})", num, row, col);
                    assert!(sgrid.add_digit(num as u8, row, col));
                    applied = true
                }
            }

            return applied;
        }

        // Check for last remaining cell in a box
        for i in (0..9).step_by(3) {
            for j in (0..9).step_by(3)  {
                applied |= find_last_remaining(sgrid, (i..i+3).flat_map(|i| (j..j+3).map(move |j| (i, j))).collect());
            }
        }

        // Check for last remaining cell in a row
        for row in 0..9 {
            applied |= find_last_remaining(sgrid, (0..9).map(|col| (row, col)).collect());
        }

        // Check for last remaining cell in a column
        for col in 0..9 {
            applied |= find_last_remaining(sgrid, (0..9).map(|row| (row, col)).collect());
        }

        return applied;
    }
}