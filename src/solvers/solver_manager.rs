use crate::{sudoku_grid::SudokuGrid, sudoku_visualizer_builder::SudokuVisualizerBuilder};

use super::sudoku_solver::{SudokuSolveMethod, SolverAction, VisualizerUpdate};


pub struct SudokuSolverManager {
    pub sgrid: SudokuGrid,
    pub current_step: usize,
    pub solvers: Vec<Box<dyn SudokuSolveMethod>>,
    pub visualizers_per_step: Vec<Vec<SudokuVisualizerBuilder>>,
}

impl SudokuSolverManager {
    pub fn new(sgrid: SudokuGrid) -> Self {
        Self {
            sgrid,
            solvers: Default::default(),
            current_step: 0,
            visualizers_per_step: Default::default(),
        }
    }

    pub fn add_solver(&mut self, solver: Box<dyn SudokuSolveMethod>) {
        self.solvers.push(solver);
    }

    pub fn solve_iteration(&mut self) -> bool {
        let mut applied_solver = false;
        self.current_step += 1;
        let mut visualizer: SudokuVisualizerBuilder = SudokuVisualizerBuilder::from_sudoku(&self.sgrid);
        // Add Pre step visualizer of the board
        {
            let mut cl = visualizer.clone();
            cl.set_title(format!("Pre step {}", self.current_step).as_str());
            self.visualizers_per_step.push(Default::default());
            self.visualizers_per_step.last_mut().unwrap().push(cl);
        }
        for solver in &self.solvers {
            let solver_result = solver.apply(&self.sgrid);
            if solver_result.is_none() { continue; }
            let (reductions, visualizer_updates) = solver_result.unwrap();

            // Apply grid reductions
            for action in reductions {
                match action {
                    SolverAction::DigitSolve(row, col, digit) => {
                        assert!(self.sgrid.add_digit(digit, row, col));
                    },
                    SolverAction::CandidateReduction(row, col, digit) => {
                        self.sgrid.candidates[row][col].remove(&digit);
                    },
                }
            }

            // Apply visualiztion additions
            for update in &visualizer_updates {
                match *update {
                    VisualizerUpdate::SetTitle(ref title) => visualizer.set_title(title),
                    VisualizerUpdate::ColorDigit(row, col, color) => visualizer.color_digit(row, col, self.sgrid.grid[row][col], color),
                    VisualizerUpdate::ColorCell(row, col, color) => visualizer.color_cell(row, col, color),
                    VisualizerUpdate::HighlightCandidate(row, col, num, color) => visualizer.color_candidate(row, col, num, color),
                }
            }
            applied_solver = true;
            break;
        }
        visualizer.set_title(format!("{} - Step {}", visualizer.title, self.current_step).as_str());
        self.visualizers_per_step.last_mut().unwrap().push(visualizer);

        // Add Post step visualizer of the board
        {
            let mut cl = SudokuVisualizerBuilder::from_sudoku(&self.sgrid);
            cl.set_title(format!("Post step {}", self.current_step).as_str());
            self.visualizers_per_step.last_mut().unwrap().push(cl);
        }
        applied_solver
    }

    pub fn generate_output_pdf(&self, file_path: &str) {

    }
}


