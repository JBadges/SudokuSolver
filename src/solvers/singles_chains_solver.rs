use std::collections::HashSet;

use super::super::sudoku_grid::SudokuGrid;
use super::sudoku_solver::SudokuSolveMethod;
use super::super::adjacency_graph::{AdjacencyGraph, BiColor};

use itertools::Itertools;

pub struct SinglesChainsSolver;

impl SudokuSolveMethod for SinglesChainsSolver {
    fn apply(&self, sgrid: &mut SudokuGrid) -> bool {
        let mut applied = false;

        for num in 1..=9 {
            let pairs = sgrid.get_conjugate_pairs(num);
            if pairs.is_empty() { continue; }

            let bicolored_graphs = AdjacencyGraph::bicolor_graphs(&pairs);

            for bicolored_graph in bicolored_graphs {
                let mut red_nodes: HashSet<(usize, usize)> = HashSet::new();
                let mut blue_nodes: HashSet<(usize, usize)> = HashSet::new();

                for (node, color) in bicolored_graph.iter() {
                    match color {
                        BiColor::Red => red_nodes.insert(*node),
                        BiColor::Blue => blue_nodes.insert(*node),
                        _ => { assert!(false, "every node should be colored"); false }
                    };
                }

                let mut process_same_color_in_unit = |color_nodes: &HashSet<(usize, usize)>| {
                    for cells in color_nodes.iter().combinations(2) {
                        let cella = cells[0];
                        let cellb = cells[1];

                        if SudokuGrid::cells_see_each_other(*cella, *cellb) {
                            for node in color_nodes {
                                if sgrid.candidates[node.0][node.1].remove(&num) {
                                    applied = true;
                                    println!("Solver [SinglesChainsSolver:SameColorInUnit] removed value {} from candidate location ({}, {})", num, node.0, node.1);
                                }
                            }
                        }
                    }
                };
                process_same_color_in_unit(&red_nodes);
                process_same_color_in_unit(&blue_nodes);

                // Cell sees both colors
                for row in 0..9 {
                    for col in 0..9 {
                        let cell = (row, col);
                        let sees_red = red_nodes.iter().any(|a| SudokuGrid::cells_see_each_other(cell, *a) && *a != cell);
                        let sees_blue = blue_nodes.iter().any(|a| SudokuGrid::cells_see_each_other(cell, *a) && *a != cell);
                        if sees_red && sees_blue && sgrid.candidates[row][col].remove(&num) {
                            applied = true;
                            println!("Solver [SinglesChainsSolver:CellSeesBothColors] removed value {} from candidate location ({}, {})", num, row, col);
                        }
                    }
                }

            }
        }

        applied
    }
}
