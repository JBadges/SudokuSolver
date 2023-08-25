use sudoku_generator::sudoku_grid::*;
use sudoku_generator::solvers::sudoku_solver::SudokuSolveMethod;
use sudoku_generator::solvers::naked_singles_solver::NakedSinglesSolver;
use sudoku_generator::solvers::naked_candidates_solver::NakedCandidatesSolver;
use sudoku_generator::solvers::hidden_candidates_solver::HiddenCandidatesSolver;
use sudoku_generator::solvers::intersection_removal_solver::IntersectionRemovalSolver;
use sudoku_generator::solvers::x_wing_solver::XWingSolver;
use sudoku_generator::solvers::y_wing_solver::YWingSolver;


fn main() {
    // let mut grid = SudokuGrid::create_sudoku_puzzle(100);
    let mut grid = SudokuGrid::from_string("093004560060003140004608309981345000347286951652070483406002890000400010029800034");
    println!("{}", grid);
    println!("{}", grid.to_number_string());
    println!("Generated puzzle with {} blanks", grid.grid.iter().flatten().filter(|&&x| x == 0).count());
    
    let mut solvers: Vec<Box<dyn SudokuSolveMethod>> = Vec::new();

    solvers.push(Box::new(NakedSinglesSolver));
    solvers.push(Box::new(NakedCandidatesSolver));
    solvers.push(Box::new(HiddenCandidatesSolver));
    solvers.push(Box::new(IntersectionRemovalSolver));
    solvers.push(Box::new(XWingSolver));
    solvers.push(Box::new(YWingSolver));

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
