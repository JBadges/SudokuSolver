use std::collections::{HashMap, HashSet};

use itertools::{iproduct, Itertools};

use crate::{sudoku_grid::SudokuGrid, sudoku_visualizer_builder::Colors, adjacency_graph::AdjacencyGraph};

use super::sudoku_solver::{SolverAction::*, SolverResult, SudokuSolveMethod, VisualizerUpdate::*};

pub struct XYChainSolver;

// An X-Chain constructs a chain of Weak & Strong links where there can never be two connected
// Weak links and it must start and end on a strong link.

impl SudokuSolveMethod for XYChainSolver {
    fn apply(&self, sgrid: &SudokuGrid) -> Option<SolverResult> {
        let bi_valued_cells: Vec<(usize, usize)> = iproduct!(0..9, 0..9).filter(|&(row, col)| sgrid.candidates[row][col].len() == 2).collect();
        let mut is_strong_link: HashMap<((usize, usize, usize), (usize, usize, usize)), bool> = Default::default();
        let mut graph = AdjacencyGraph::new();
        // Construct graph from bi_valued_cells
        for pair in bi_valued_cells.iter().combinations(2) {
            let (cell1, cell2) = (*pair[0], *pair[1]);

            if !SudokuGrid::cells_see_each_other(cell1, cell2) { continue; }

            let cell1_candidates = &sgrid.candidates[cell1.0][cell1.1];
            let cell2_candidates = &sgrid.candidates[cell2.0][cell2.1];

            let candidates_in_common: HashSet<usize> = cell1_candidates.intersection(cell2_candidates).cloned().collect();
            if candidates_in_common.is_empty() { continue; }
            
            let shared_cells: Vec<(usize, usize)> = SudokuGrid::generate_cells_seen_from_cord(cell1).intersection(&SudokuGrid::generate_cells_seen_from_cord(cell2)).cloned().collect();
            for candidate in candidates_in_common {
                let mut strong_link = true;
                for &common_cell in &shared_cells { 
                    if common_cell == cell1 || common_cell == cell2 { continue; }

                    if sgrid.candidates[common_cell.0][common_cell.1].contains(&candidate) {
                        strong_link = false;
                        break;
                    }
                }
                graph.add_edge((cell1.0, cell1.1, candidate), (cell2.0, cell2.1, candidate));
                is_strong_link.insert(((cell1.0, cell1.1, candidate), (cell2.0, cell2.1, candidate)), strong_link);
                is_strong_link.insert(((cell2.0, cell2.1, candidate), (cell1.0, cell1.1, candidate)), strong_link);
            }

            // Add inner strong links in the cells
            let mut cell1_iter = cell1_candidates.iter();
            if let (Some(&a), Some(&b)) = (cell1_iter.next(), cell1_iter.next()) {
                graph.add_edge((cell1.0, cell1.1, a), (cell1.0, cell1.1, b));
                graph.add_edge((cell1.0, cell1.1, b), (cell1.0, cell1.1, a));
                is_strong_link.insert(((cell1.0, cell1.1, a), (cell1.0, cell1.1, b)), true);
                is_strong_link.insert(((cell1.0, cell1.1, b), (cell1.0, cell1.1, a)), true);
            } else {
                panic!("All Bi-valued cells should have two candidates");
            }

            let mut cell2_iter: std::collections::hash_set::Iter<'_, usize> = cell2_candidates.iter();
            if let (Some(&a), Some(&b)) = (cell2_iter.next(), cell2_iter.next()) {
                graph.add_edge((cell2.0, cell2.1, a), (cell2.0, cell2.1, b));
                graph.add_edge((cell2.0, cell2.1, b), (cell2.0, cell2.1, a));
                is_strong_link.insert(((cell2.0, cell2.1, a), (cell2.0, cell2.1, b)), true);
                is_strong_link.insert(((cell2.0, cell2.1, b), (cell2.0, cell2.1, a)), true);
            } else {
                panic!("All Bi-valued cells should have two candidates");
            }
        }

        for chain_length in (2..=10).step_by(2) {

            // Go through the graph starting with only strong links and either going strong->strong or strong->weak->strong for a length of CHAIN_LENGTH
            for &(row, col) in &bi_valued_cells {
                for &candidate in &sgrid.candidates[row][col] {
                    let start_node = (row, col, candidate);
                    let chains = XYChainSolver::find_chains_from_node(&sgrid, start_node, &graph, &is_strong_link, chain_length);
                    for chain in chains {
                        let mut viualizer_updates = Vec::new();
                        let mut reductions = Vec::new();
                        viualizer_updates.push(SetTitle(format!("XY-Chain")));

                        let first = chain.first().unwrap();
                        let last = chain.last().unwrap();

                        // If the candidate from the start is not the same as the one from the end, or if it's used in the first or last cell, then continue.
                        if first.2 != last.2 {
                            continue;
                        }

                        let candidate_for_removal = first.2;

                        let cells_to_check: Vec<(usize, usize)> =
                            SudokuGrid::generate_cells_seen_from_cord((first.0, first.1))
                                .intersection(&SudokuGrid::generate_cells_seen_from_cord((
                                    last.0, last.1,
                                )))
                                .cloned()
                                .collect();
                        for (row, col) in cells_to_check {
                            if chain.contains(&(row, col, first.2)) || chain.contains(&(row, col, last.2)) {
                                continue;
                            }
                            viualizer_updates.push(ColorCell(
                                row,
                                col,
                                Colors::CELL_MARKED_FOR_CANDIDATE_REMOVEAL,
                            ));
                            if sgrid.candidates[row][col].contains(&candidate_for_removal) {
                                reductions.push(CandidateReduction(row, col, candidate_for_removal));
                                viualizer_updates.push(ColorCandidate(
                                    row,
                                    col,
                                    candidate_for_removal,
                                    Colors::CANDIDATE_MARKED_FOR_REMOVAL,
                                ));
                            }
                        }

                        for i in 0..chain.len() {
                            let candidate_color = match i % 2 == 0 {
                                true => Colors::CHAIN_BLUE,
                                false => Colors::CHAIN_RED,
                            };
                            viualizer_updates.push(BackgroundCandidate(
                                chain[i].0,
                                chain[i].1,
                                chain[i].2,
                                candidate_color,
                            ));
                            if i != chain.len() - 1 {
                                let chain_color = match is_strong_link.get(&(chain[i], chain[i+1])) {
                                    Some(true) => Colors::CHAIN_STRONG,
                                    Some(false) => Colors::CHAIN_WEAK,
                                    _ => panic!("This shouldn't be possible.")
                                };
                                viualizer_updates.push(CreateChain(
                                    chain[i].0,
                                    chain[i].1,
                                    chain[i].2,
                                    chain[i + 1].0,
                                    chain[i + 1].1,
                                    chain[i + 1].2,
                                    chain_color,
                                ));
                            }
                        }

                        if !reductions.is_empty() { return Some((reductions, viualizer_updates)); }
                    }
                }
            }
        }
        None
    }
}

impl XYChainSolver {
    fn find_chains_from_node(
        sgrid: &SudokuGrid,
        start_node: (usize, usize, usize),
        conjugate_pairs: &AdjacencyGraph,
        is_strong_link: &HashMap<((usize, usize, usize), (usize, usize, usize)), bool>,
        chain_length: usize
    ) -> Vec<Vec<(usize, usize, usize)>> {
        let mut chains = Vec::new();
        let mut visited = HashSet::new();
        XYChainSolver::dfs(sgrid, start_node, &mut Vec::new(), &mut chains, &mut visited, conjugate_pairs, is_strong_link, chain_length);
        chains
    }
    
    fn dfs(
        sgrid: &SudokuGrid,
        mut current_node: (usize, usize, usize),
        current_chain: &mut Vec<(usize, usize, usize)>,
        chains: &mut Vec<Vec<(usize, usize, usize)>>,
        visited: &mut HashSet<(usize, usize, usize)>,
        graph: &AdjacencyGraph,
        is_strong_link: &HashMap<((usize, usize, usize), (usize, usize, usize)), bool>,
        chain_length: usize
    ) {
        if current_chain.len() == chain_length {
            chains.push(current_chain.clone());
            return;
        }
    
        visited.insert(current_node);
        current_chain.push(current_node);
    
        // Prioritize the other candidate in the same cell
        let &other_candidate = sgrid.candidates[current_node.0][current_node.1]
            .iter()
            .find(|&&candidate| candidate != current_node.2)
            .unwrap();
        current_node = (current_node.0, current_node.1, other_candidate);

        
        visited.insert(current_node);
        current_chain.push(current_node);
    
        
        // Explore other neighbors
        if let Some(neighbors) = graph.neighbors(current_node) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    XYChainSolver::dfs(sgrid, neighbor, current_chain, chains, visited, graph, is_strong_link, chain_length);
                }
            }
        }
    
        visited.remove(&current_chain.pop().unwrap());
        visited.remove(&current_chain.pop().unwrap());
    }
      
}
