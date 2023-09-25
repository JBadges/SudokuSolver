use std::collections::{HashMap, HashSet};

use crate::{sudoku_grid::SudokuGrid, sudoku_visualizer_builder::Colors, adjacency_graph::AdjacencyGraph};

use super::sudoku_solver::{SolverAction::*, SolverResult, SudokuSolveMethod, VisualizerUpdate::*};

pub struct XChainSolver;

// An X-Chain constructs a chain of Weak & Strong links where there can never be two connected
// Weak links and it must start and end on a strong link.

impl SudokuSolveMethod for XChainSolver {
    fn apply(&self, sgrid: &SudokuGrid) -> Option<SolverResult> {
        for chain_length in (3..=9).step_by(2) {
            for num in 1..=9 {
                let mut conjugate_pairs= sgrid.get_conjugate_pairs(num);
                let mut is_strong_link: HashMap<((usize, usize, usize), (usize, usize, usize)), bool> = conjugate_pairs.edges.iter()
                    .flat_map(|(&key, ends)| ends
                        .iter()
                        .map(move |&link| ((key, link), true)))
                    .collect();

                
                // Construct the full adjacency graph from only strong links to include all weak links
                let keys: Vec<_> = conjugate_pairs.edges.keys().cloned().collect();
                for key in keys {
                    for (row, col) in SudokuGrid::generate_cells_seen_from_cord((key.0, key.1)) {
                        if sgrid.candidates[row][col].contains(&num) {
                            if !conjugate_pairs.edges.get(&key).map_or(false, |neighbors| neighbors.contains(&(row, col, num))) {
                                conjugate_pairs.add_edge(key, (row, col, num));
                                is_strong_link.insert((key, (row, col, num)), false);
                                is_strong_link.insert(((row, col, num), key), false);
                            }                                                    
                        }
                    }
                }

                // Go through the graph starting with only strong links and either going strong->strong or strong->weak->strong for a length of CHAIN_LENGTH
                for start_node in is_strong_link.iter().filter(|&(_, &value)| value).flat_map(|(&key, _)| vec![key.0, key.1]) {
                    let chains = XChainSolver::find_chains_from_node(start_node, &conjugate_pairs, &is_strong_link, chain_length + 1);
                    for chain in chains {
                        let mut viualizer_updates = Vec::new();
                        let mut reductions = Vec::new();
                        debug_assert!(chain_length == chain.len() - 1, "Chain found and chain length aren't the same.");
                        viualizer_updates.push(SetTitle(format!("X-Chain length {}", chain.len() - 1)));
                        let first = chain.first().unwrap();
                        let last = chain.last().unwrap();
                        let cells_to_check: Vec<(usize, usize)> =
                            SudokuGrid::generate_cells_seen_from_cord((first.0, first.1))
                                .intersection(&SudokuGrid::generate_cells_seen_from_cord((
                                    last.0, last.1,
                                )))
                                .cloned()
                                .collect();
                        for (row, col) in cells_to_check {
                            if chain.contains(&(row, col, num)) {
                                continue;
                            }
                            viualizer_updates.push(ColorCell(
                                row,
                                col,
                                Colors::CELL_MARKED_FOR_CANDIDATE_REMOVEAL,
                            ));
                            if sgrid.candidates[row][col].contains(&num) {
                                reductions.push(CandidateReduction(row, col, num));
                                viualizer_updates.push(ColorCandidate(
                                    row,
                                    col,
                                    num,
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
                                num,
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
                                    num,
                                    chain[i + 1].0,
                                    chain[i + 1].1,
                                    num,
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
impl XChainSolver {
    fn find_chains_from_node(
        start_node: (usize, usize, usize),
        conjugate_pairs: &AdjacencyGraph,
        is_strong_link: &HashMap<((usize, usize, usize), (usize, usize, usize)), bool>,
        chain_length: usize
    ) -> Vec<Vec<(usize, usize, usize)>> {
        let mut chains = Vec::new();
        let mut visited = HashSet::new();
        XChainSolver::dfs(start_node, &mut Vec::new(), &mut chains, &mut visited, conjugate_pairs, is_strong_link, chain_length);
        chains
    }
    
    fn dfs(
        current_node: (usize, usize, usize),
        current_chain: &mut Vec<(usize, usize, usize)>,
        chains: &mut Vec<Vec<(usize, usize, usize)>>,
        visited: &mut HashSet<(usize, usize, usize)>,
        graph: &AdjacencyGraph,
        is_strong_link: &HashMap<((usize, usize, usize), (usize, usize, usize)), bool>,
        chain_length: usize
    ) {
        if current_chain.len() == chain_length {
            // Ensure the chain ends with a strong link
            if let Some(&val) = is_strong_link.get(&(current_chain.last().unwrap().clone(), current_chain[current_chain.len() - 2])) {
                if val {
                    chains.push(current_chain.clone());
                }
            }
            
            return;
        }
    
        visited.insert(current_node);
        current_chain.push(current_node);
    
        if let Some(neighbors) = graph.neighbors(current_node) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    let link = is_strong_link.get(&(current_node, neighbor)).unwrap_or(&false);
                    // We can always add a strong link
                    if *link {
                        XChainSolver::dfs(neighbor, current_chain, chains, visited, graph, is_strong_link, chain_length);
                    } else {
                        // Ensure the link before was strong
                        if current_chain.len() >= 2 && current_chain.len() % 2 == 0 {
                            if let Some(&val) = is_strong_link.get(&(current_chain.last().unwrap().clone(), current_chain[current_chain.len() - 2])) {
                                if val {
                                    XChainSolver::dfs(neighbor, current_chain, chains, visited, graph, is_strong_link, chain_length);
                                }
                            }
                        }
                    }
                }
            }
        }
    
        current_chain.pop();
        visited.remove(&current_node);
    }
      
}
