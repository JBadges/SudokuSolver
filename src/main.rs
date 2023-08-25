use sudoku_generator::sudoku_grid::*;
use sudoku_generator::solvers::sudoku_solver::SudokuSolveMethod;
use sudoku_generator::solvers::naked_singles_solver::NakedSinglesSolver;
use sudoku_generator::solvers::naked_candidates_solver::NakedCandidatesSolver;
use sudoku_generator::solvers::hidden_candidates_solver::HiddenCandidatesSolver;


fn main() {
    // let mut grid = SudokuGrid::create_sudoku_puzzle(100);
    let mut grid = SudokuGrid::from_string("300000000970010000600583000200000900500621003008000005000435002000090056000000001");
    println!("{}", grid);
    println!("{}", grid.to_number_string());
    println!("Generated puzzle with {} blanks", grid.grid.iter().flatten().filter(|&&x| x == 0).count());
    
    let mut solvers: Vec<Box<dyn SudokuSolveMethod>> = Vec::new();

    solvers.push(Box::new(NakedSinglesSolver));
    solvers.push(Box::new(NakedCandidatesSolver));
    solvers.push(Box::new(HiddenCandidatesSolver));

    let mut applied = true;
    while applied {
        for solver in &solvers {
            applied = solver.apply(&mut grid);
            if applied {
                // println!("{}", grid);
                break;
            }
        }
    }
    println!("Unable to apply more move with final board state:");
    println!("{}", grid);
}
