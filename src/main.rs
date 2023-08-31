use sudoku_generator::solvers::medusa_3d_solver::Medusa3DSolver;
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

use sudoku_generator::sudoku_latex_builder::*;

fn main() {
    let mut builder = SudokuLatexBuilder::new();
    builder.add_grid();
    builder.color_candidate(0, 0, 1, "red");
    builder.color_candidate(1, 1, 9, "blue");
    builder.add_chain((0, 0, 1), (1, 1, 9), "green");
    let _ = builder.build("imgs/sudoku_gen");

    // let mut grid = SudokuGrid::create_sudoku_puzzle(100);
    // let mut grid = SudokuGrid::from_string("093824560085600002206075008321769845000258300578040296850016723007082650002507180");
    // println!("{}", grid);
    // println!("{}", grid.to_number_string());
    // println!("Generated puzzle with {} blanks", grid.grid.iter().flatten().filter(|&&x| x == 0).count());
    
    // let mut solvers: Vec<Box<dyn SudokuSolveMethod>> = Vec::new();

    // solvers.push(Box::new(NakedSinglesSolver));
    // solvers.push(Box::new(NakedCandidatesSolver));
    // solvers.push(Box::new(HiddenCandidatesSolver));
    // solvers.push(Box::new(IntersectionRemovalSolver));
    // solvers.push(Box::new(XWingSolver));
    // solvers.push(Box::new(SinglesChainsSolver));
    // solvers.push(Box::new(YWingSolver));
    // solvers.push(Box::new(SwordfishSolver));
    // solvers.push(Box::new(JellyfishSolver));
    // solvers.push(Box::new(Medusa3DSolver));

    // let mut applied = true;
    // while applied {
    //     for solver in &solvers {
    //         applied = solver.apply(&mut grid);
    //         if applied {
    //             break;
    //         }
    //     }
    // }
    // println!("Unable to apply more solvers. Final board state:");
    // println!("{}", grid);
}
