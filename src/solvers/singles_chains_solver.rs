use std::collections::HashSet;

use crate::sudoku_visualizer_builder::Colors;

use super::super::sudoku_grid::SudokuGrid;
use super::sudoku_solver::*;
use super::super::adjacency_graph::{AdjacencyGraph, BiColor};

use itertools::{Itertools, iproduct};
use raylib::prelude::Color;

pub struct SinglesChainsSolver;

impl SudokuSolveMethod for SinglesChainsSolver {
    fn apply(&self, sgrid: &SudokuGrid) -> Option<SolverResult> {
        for num in 1..=9 {
            let mut visualizer_updates = Vec::new();
            let mut reductions = Vec::new();
            visualizer_updates.push(VisualizerUpdate::SetTitle("Singles Chain".to_string()));

            let pairs = sgrid.get_conjugate_pairs(num);
            if pairs.is_empty() { continue; }

            let bicolored_graphs = AdjacencyGraph::bicolor_graphs(&pairs);

            let get_color_of_node = |(row, col)| -> Color {
                for graph in &bicolored_graphs {
                    if graph.contains_key(&(row, col)) { 
                        return match  graph[&(row, col)] {
                            BiColor::Blue => Colors::CHAIN_BLUE,
                            BiColor::Red => Colors::CHAIN_RED,
                            BiColor::None => panic!("Uncolored node in graph."),
                    } }
                }
                panic!("Error, couldn't find node in graphs.")
            };

            for &(row, col) in &pairs.nodes() {
                for &(rowb, colb) in pairs.neighbors((row, col)).unwrap() {
                    visualizer_updates.push(VisualizerUpdate::CreateChain(row, col, num, rowb, colb, num, Colors::CHAIN_COLOR));
                }
                visualizer_updates.push(VisualizerUpdate::ColorCell(row, col, Colors::CELL_USED_TO_DETERMINE_SOLUTION));
                visualizer_updates.push(VisualizerUpdate::BackgroundCandidate(row, col, num, get_color_of_node((row, col))));
            }

            for bicolored_graph in bicolored_graphs {
                let mut red_nodes: HashSet<(usize, usize)> = HashSet::new();
                let mut blue_nodes: HashSet<(usize, usize)> = HashSet::new();

                for (node, color) in bicolored_graph.iter() {
                    match color {
                        BiColor::Red => red_nodes.insert(*node),
                        BiColor::Blue => blue_nodes.insert(*node),
                        _ => panic!("every node should be colored"),
                    };
                }

                let mut process_same_color_in_unit = |color_nodes: &HashSet<(usize, usize)>| {
                    for cells in color_nodes.iter().combinations(2) {
                        let cella = *cells[0];
                        let cellb = *cells[1];

                        if SudokuGrid::cells_see_each_other(cella, cellb) {
                            for &(row, col) in color_nodes {
                                if sgrid.candidates[row][col].contains(&num) {
                                    visualizer_updates.push(VisualizerUpdate::ColorCandidate(row, col, num, Colors::CANDIDATE_MARKED_FOR_REMOVAL));
                                    visualizer_updates.push(VisualizerUpdate::ColorCell(row, col, Colors::CELL_MARKED_FOR_CANDIDATE_REMOVEAL));
                                    reductions.push(SolverAction::CandidateReduction(row, col, num));
                                }
                            }
                        }
                    }
                };
                process_same_color_in_unit(&red_nodes);
                process_same_color_in_unit(&blue_nodes);

                // Cell sees both colors
                for (row, col) in iproduct!(0..9, 0..9).collect::<Vec<(usize, usize)>>() {
                    let sees_red = red_nodes.iter().any(|&red_cell| SudokuGrid::cells_see_each_other((row, col), red_cell) && red_cell != (row, col));
                    let sees_blue = blue_nodes.iter().any(|&blue_cell| SudokuGrid::cells_see_each_other((row, col), blue_cell) && blue_cell != (row, col));
                    if sees_red && sees_blue && sgrid.candidates[row][col].contains(&num) {
                        visualizer_updates.push(VisualizerUpdate::ColorCandidate(row, col, num, Colors::CANDIDATE_MARKED_FOR_REMOVAL));
                        visualizer_updates.push(VisualizerUpdate::ColorCell(row, col, Colors::CELL_MARKED_FOR_CANDIDATE_REMOVEAL));
                        reductions.push(SolverAction::CandidateReduction(row, col, num));
                    }
            
                }

            }
            if !reductions.is_empty() { return Some((reductions, visualizer_updates)); }
        }

        None
    }
}
