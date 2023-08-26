use sudoku_generator::sudoku_grid::*;
use sudoku_generator::solvers::sudoku_solver::SudokuSolveMethod;
use sudoku_generator::solvers::naked_singles_solver::NakedSinglesSolver;
use sudoku_generator::solvers::naked_candidates_solver::NakedCandidatesSolver;
use sudoku_generator::solvers::hidden_candidates_solver::HiddenCandidatesSolver;
use sudoku_generator::solvers::intersection_removal_solver::IntersectionRemovalSolver;
use sudoku_generator::solvers::x_wing_solver::XWingSolver;
use sudoku_generator::solvers::singles_chains_solver::SinglesChainsSolver;
use sudoku_generator::solvers::y_wing_solver::YWingSolver;
use sudoku_generator::solvers::swordfish_solver::SwordfishSolver;
use sudoku_generator::solvers::jellyfish_solver::JellyfishSolver;


fn main() {
    // let mut grid = SudokuGrid::create_sudoku_puzzle(100);
    let mut grid = SudokuGrid::from_string("024090008800402900719000240075804300240900587038507604082000059007209003490050000");
    println!("{}", grid);
    println!("{}", grid.to_number_string());
    println!("Generated puzzle with {} blanks", grid.grid.iter().flatten().filter(|&&x| x == 0).count());
    
    let mut solvers: Vec<Box<dyn SudokuSolveMethod>> = Vec::new();

    solvers.push(Box::new(NakedSinglesSolver));
    solvers.push(Box::new(NakedCandidatesSolver));
    solvers.push(Box::new(HiddenCandidatesSolver));
    solvers.push(Box::new(IntersectionRemovalSolver));
    solvers.push(Box::new(XWingSolver));
    solvers.push(Box::new(SinglesChainsSolver));
    solvers.push(Box::new(YWingSolver));
    solvers.push(Box::new(SwordfishSolver));
    solvers.push(Box::new(JellyfishSolver));

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
    println!("Unable to apply more solvers. Final board state:");
    println!("{}", grid);
}
