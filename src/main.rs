use sudoku_generator::sudoku_grid::*;
use sudoku_generator::solvers::sudoku_solver::SudokuSolveMethod;
use sudoku_generator::solvers::naked_singles_solver::NakedSinglesSolver;
use sudoku_generator::solvers::naked_candidates_solver::NakedCandidatesSolver;
use sudoku_generator::solvers::hidden_candidates_solver::HiddenCandidatesSolver;
use sudoku_generator::solvers::intersection_removal_solver::IntersectionRemovalSolver;


fn main() {
    // let mut grid = SudokuGrid::create_sudoku_puzzle(100);
    let mut grid = SudokuGrid::from_string("000921003009000060000000500080403006007000800500700040003000000020000700800195000");
    println!("{}", grid);
    println!("{}", grid.to_number_string());
    println!("Generated puzzle with {} blanks", grid.grid.iter().flatten().filter(|&&x| x == 0).count());
    
    let mut solvers: Vec<Box<dyn SudokuSolveMethod>> = Vec::new();

    solvers.push(Box::new(NakedSinglesSolver));
    solvers.push(Box::new(NakedCandidatesSolver));
    solvers.push(Box::new(HiddenCandidatesSolver));
    solvers.push(Box::new(IntersectionRemovalSolver));

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
