use raylib::prelude::Color;

use crate::sudoku_grid::SudokuGrid;

pub enum SolverAction {
    CandidateReduction(usize, usize, usize),
    DigitSolve(usize, usize, usize),
}

pub enum VisualizerUpdate {
    SetTitle(String),
    ColorDigit(usize, usize, Color),
    ColorCell(usize, usize, Color),
    ColorCandidate(usize, usize, usize, Color),
    BackgroundCandidate(usize, usize, usize, Color),
    CreateChain(usize, usize, usize, usize, usize, usize, Color),
}

pub type SolverResult = (Vec<SolverAction>, Vec<VisualizerUpdate>);

pub trait SudokuSolveMethod {
    fn apply(&self, sgrid: &SudokuGrid) -> Option<SolverResult>;
}
