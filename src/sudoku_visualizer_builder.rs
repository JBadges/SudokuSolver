use std::collections::HashMap;
use crate::sudoku_grid::SudokuGrid;
use raylib::prelude::*;

type Coordinate = (usize, usize);
type CandidateCoordinate = (usize, usize, usize);
type Chain = (CandidateCoordinate, CandidateCoordinate);


pub struct Colors;

impl Colors {
    pub const SOLVED_DIGIT: Color = Color::new(12, 94, 34, 255);
    pub const CANDIDATE_MARKED_FOR_REMOVAL: Color = Color::new(230, 14, 68, 255);
    pub const CELL_USED_TO_DETERMINE_SOLUTION: Color = Color::new(196, 145, 69, 255);
    pub const DIGIT_USED_TO_DETERMINE_SOLUTION: Color = Color::new(204, 242, 15, 255);
    pub const CELL_MARKED_FOR_CANDIDATE_REMOVEAL: Color = Color::new(175, 203, 204, 255);
}

#[derive(Clone)]
pub struct SudokuVisualizerBuilder {
    pub content: String,
    pub candidates: HashMap<CandidateCoordinate, Color>,
    pub digits: HashMap<Coordinate, (usize, Color)>,

    pub candidates_highlights: HashMap<CandidateCoordinate, Color>,
    pub digits_highlights: HashMap<Coordinate, Color>,
    pub cell_highlights: HashMap<Coordinate, Color>,

    pub chains: HashMap<Chain, Color>,
    pub title: String, 
}

impl SudokuVisualizerBuilder {
    pub fn new() -> Self {
        SudokuVisualizerBuilder {
            content: String::new(),
            candidates: HashMap::new(),
            candidates_highlights: HashMap::new(),
            digits: HashMap::new(),
            digits_highlights: HashMap::new(),
            cell_highlights: HashMap::new(),
            chains: HashMap::new(),
            title: "Solver".to_string(),
        }
    }

    pub fn from_sudoku(sgrid: &SudokuGrid) -> Self {
        let mut visualizer = SudokuVisualizerBuilder::new();
        for row in 0..9 {
            for col in 0..9 {
                if sgrid.grid[row][col] != 0 {
                    visualizer.digits.insert((row, col), (sgrid.grid[row][col], Color::GRAY));
                } else {
                    for candidate in &sgrid.candidates[row][col] {
                        visualizer.candidates.insert((row, col, *candidate), Color::BLACK);
                    }
                }
            }
        }
        visualizer
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
    }

    pub fn color_digit(&mut self, row: usize, col: usize, num: usize, color: Color) {
        self.digits.insert((row, col), (num, color));
    }

    pub fn color_candidate(&mut self, row: usize, col: usize, num: usize, color: Color) {
        self.candidates.insert((row, col, num), color);
    }

    pub fn highlight_candidate(&mut self, row: usize, col: usize, num: usize, color: Color) {
        self.candidates_highlights.insert((row, col, num), color);
    }

    pub fn color_cell(&mut self, row: usize, col: usize, color: Color) {
        self.cell_highlights.insert((row, col), color);
    }

    pub fn add_chain(&mut self, row_from: usize, col_from: usize, num_from: usize, row_to: usize, col_to: usize, num_to: usize, color: Color) {
        self.chains.insert(((row_from, col_from, num_from), (row_to, col_to, num_to)), color);
    }

}