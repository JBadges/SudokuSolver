use std::collections::{HashMap, HashSet};

use itertools::{iproduct, Itertools};
use raylib::prelude::Color;

use crate::adjacency_graph::{AdjacencyGraph, BiColor};
use crate::sudoku_visualizer_builder::Colors;

use super::super::sudoku_grid::*;
use super::sudoku_solver::*;

pub struct Medusa3DSolver;

impl SudokuSolveMethod for Medusa3DSolver {
    fn apply(&self, sgrid: &SudokuGrid) -> Option<SolverResult> {
        let graph = (1..=9).fold(AdjacencyGraph::new(), |mut acc, num| {
            acc.merge_on_bivalue(sgrid.get_conjugate_pairs(num), &sgrid);
            acc
        });

        let bicolored_graphs = AdjacencyGraph::bicolor_graphs(&graph);

        for bicolored_graph in bicolored_graphs.iter() {
            let mut visualizer_updates = Vec::new();
            let mut reductions = Vec::new();

            let get_color_of_node = |(row, col, num)| -> Color {
                if bicolored_graph.contains_key(&(row, col, num)) {
                    match bicolored_graph[&(row, col, num)] {
                        BiColor::Blue => Colors::CHAIN_BLUE,
                        BiColor::Red => Colors::CHAIN_RED,
                        BiColor::None => panic!("Uncolored node in graph."),
                    }
                } else {
                    panic!("Error, couldn't find node in graphs.")
                }
            };

            for &(row, col, num) in &graph.nodes() {
                if !bicolored_graph.contains_key(&(row, col, num)) {
                    continue;
                }
                for &(rowb, colb, numb) in graph.neighbors((row, col, num)).unwrap() {
                    visualizer_updates.push(VisualizerUpdate::CreateChain(
                        row,
                        col,
                        num,
                        rowb,
                        colb,
                        numb,
                        Colors::CHAIN_COLOR,
                    ));
                }
                visualizer_updates.push(VisualizerUpdate::ColorCell(
                    row,
                    col,
                    Colors::CELL_USED_TO_DETERMINE_SOLUTION,
                ));
                visualizer_updates.push(VisualizerUpdate::BackgroundCandidate(
                    row,
                    col,
                    num,
                    get_color_of_node((row, col, num)),
                ));
            }

            // Rule 1 Twice in a cell
            {
                let conflicting_color = {
                    let mut cell_colors: HashMap<(usize, usize), BiColor> = HashMap::new();
                    let mut result = None;
                    for (&(row, col, _num), &color) in bicolored_graph.iter() {
                        if let Some(&existing_color) = cell_colors.get(&(row, col)) {
                            if existing_color == color {
                                result = Some(color);
                                break;
                            }
                        } else {
                            cell_colors.insert((row, col), color);
                        }
                    }
                    result
                };

                if let Some(color) = conflicting_color {
                    visualizer_updates.push(VisualizerUpdate::SetTitle(
                        "3d Medusa: Twice in a Cell".to_string(),
                    ));
                    for (&(row, col, num), &c) in bicolored_graph.iter() {
                        if c == color {
                            visualizer_updates.push(VisualizerUpdate::ColorCandidate(
                                row,
                                col,
                                num,
                                Colors::CANDIDATE_MARKED_FOR_REMOVAL,
                            ));
                            reductions.push(SolverAction::CandidateReduction(row, col, num));
                        }
                    }
                    return Some((reductions, visualizer_updates));
                }
            }

            // Rule 2 Twice in a Unit
            {
                let conflicting_color = {
                    let mut result = None;
                    for cell_pair in bicolored_graph.iter().combinations(2) {
                        let (&(rowa, cola, numa), &colora) = cell_pair[0];
                        let (&(rowb, colb, numb), &colorb) = cell_pair[1];

                        if colora != colorb {
                            continue;
                        }
                        if numa != numb {
                            continue;
                        }
                        if SudokuGrid::cells_see_each_other((rowa, cola), (rowb, colb)) {
                            result = Some(colora);
                            break;
                        }
                    }
                    result
                };

                if let Some(color) = conflicting_color {
                    visualizer_updates.push(VisualizerUpdate::SetTitle(
                        "3d Medusa: Twice in a Unit".to_string(),
                    ));
                    for (&(row, col, num), &c) in bicolored_graph.iter() {
                        if c == color {
                            visualizer_updates.push(VisualizerUpdate::ColorCandidate(
                                row,
                                col,
                                num,
                                Colors::CANDIDATE_MARKED_FOR_REMOVAL,
                            ));
                            reductions.push(SolverAction::CandidateReduction(row, col, num));
                        }
                    }
                    return Some((reductions, visualizer_updates));
                }
            }

            // Rule 3 Two colours in a cell
            {
                visualizer_updates.push(VisualizerUpdate::SetTitle(
                    "3d Medusa: Two colours in a cell".to_string(),
                ));
                for (row, col) in bicolored_graph
                    .keys()
                    .map(|&(rowb, colb, _)| (rowb, colb))
                    .unique()
                {
                    let matching_keys: Vec<_> = bicolored_graph
                        .keys()
                        .filter(|&&(rowb, colb, _)| rowb == row && colb == col)
                        .cloned()
                        .collect();

                    if matching_keys.len() == 0 {
                        continue;
                    }

                    let &first_value = bicolored_graph.get(&matching_keys[0]).unwrap();

                    let all_homogeneous = matching_keys
                        .iter()
                        .all(|&key| *(bicolored_graph.get(&key).unwrap()) == first_value);

                    if !all_homogeneous {
                        let candidates_not_in_graph: Vec<usize> = sgrid.candidates[row][col]
                            .iter()
                            .filter(|&&candidate| {
                                !bicolored_graph.contains_key(&(row, col, candidate))
                            })
                            .cloned()
                            .collect();
                        for candidate in candidates_not_in_graph {
                            visualizer_updates.push(VisualizerUpdate::ColorCell(
                                row,
                                col,
                                Colors::CELL_MARKED_FOR_CANDIDATE_REMOVEAL,
                            ));
                            visualizer_updates.push(VisualizerUpdate::ColorCandidate(
                                row,
                                col,
                                candidate,
                                Colors::CANDIDATE_MARKED_FOR_REMOVAL,
                            ));
                            reductions.push(SolverAction::CandidateReduction(row, col, candidate));
                        }
                    }
                }
                if !reductions.is_empty() {
                    return Some((reductions, visualizer_updates));
                }
            }

            // Rule 4 Two colours 'elsewhere'
            {
                visualizer_updates.push(VisualizerUpdate::SetTitle(
                    "3d Medusa: Two colours 'elsewhere'".to_string(),
                ));
                for num in 1..=9 {
                    let mut temp_map: HashMap<(usize, usize), BiColor> = HashMap::new();

                    for (&(row, col, n), &color) in bicolored_graph.iter() {
                        if n == num {
                            temp_map.insert((row, col), color);
                        }
                    }

                    for (&(row1, col1), &color1) in &temp_map {
                        for (&(row2, col2), &color2) in &temp_map {
                            if (row1, col1) != (row2, col2) && color1 != color2 {
                                let cells_seen_by_both: Vec<(usize, usize)> =
                                    SudokuGrid::generate_cells_seen_from_cord((row1, col1))
                                        .intersection(&SudokuGrid::generate_cells_seen_from_cord((
                                            row2, col2,
                                        )))
                                        .cloned()
                                        .collect();
                                for (row, col) in cells_seen_by_both {
                                    if (row, col) == (row1, col1) || (row, col) == (row2, col2) {
                                        continue;
                                    }
                                    if sgrid.candidates[row][col].contains(&num) {
                                        visualizer_updates.push(VisualizerUpdate::ColorCell(
                                            row,
                                            col,
                                            Colors::CELL_MARKED_FOR_CANDIDATE_REMOVEAL,
                                        ));
                                        visualizer_updates.push(VisualizerUpdate::ColorCandidate(
                                            row,
                                            col,
                                            num,
                                            Colors::CANDIDATE_MARKED_FOR_REMOVAL,
                                        ));
                                        reductions
                                            .push(SolverAction::CandidateReduction(row, col, num));
                                    }
                                }
                            }
                        }
                    }
                }
                if !reductions.is_empty() {
                    return Some((reductions, visualizer_updates));
                }
            }

            // Rule 5 Two colours Unit + Cell
            {
                visualizer_updates.push(VisualizerUpdate::SetTitle(
                    "3d Medusa: Two colours Unit + Cell".to_string(),
                ));
                for (&(row, col, _), &color) in bicolored_graph.iter() {
                    for &candidate in &sgrid.candidates[row][col] {
                        if bicolored_graph.contains_key(&(row, col, candidate)) {
                            continue;
                        }

                        for (&(rowb, colb, numb), &colorb) in bicolored_graph.iter() {
                            if row == rowb && col == colb {
                                continue;
                            }

                            if color != colorb
                                && numb == candidate
                                && SudokuGrid::cells_see_each_other((row, col), (rowb, colb))
                            {
                                visualizer_updates.push(VisualizerUpdate::ColorCell(
                                    row,
                                    col,
                                    Colors::CELL_MARKED_FOR_CANDIDATE_REMOVEAL,
                                ));
                                visualizer_updates.push(VisualizerUpdate::ColorCandidate(
                                    row,
                                    col,
                                    candidate,
                                    Colors::CANDIDATE_MARKED_FOR_REMOVAL,
                                ));
                                reductions
                                    .push(SolverAction::CandidateReduction(row, col, candidate));
                            }
                        }
                    }
                }
                if !reductions.is_empty() {
                    return Some((reductions, visualizer_updates));
                }
            }

            // Rule 6 Cell Emptied by Color
            {
                let conflicting_color = {
                    let mut result = None;

                    for uncolored_cell in iproduct!(0..9, 0..9).filter(|&(row, col)| {
                        !bicolored_graph
                            .keys()
                            .any(|&(rowb, colb, _)| row == rowb && col == colb)
                    }) {
                        let mut candidate_colors: HashMap<usize, HashSet<BiColor>> = HashMap::new();
                        for cell_seen_by_uncolored_cell in
                            SudokuGrid::generate_cells_seen_from_cord(uncolored_cell)
                        {
                            for &candidate in &sgrid.candidates[uncolored_cell.0][uncolored_cell.1]
                            {
                                if let Some(&color) = bicolored_graph.get(&(
                                    cell_seen_by_uncolored_cell.0,
                                    cell_seen_by_uncolored_cell.1,
                                    candidate,
                                )) {
                                    candidate_colors
                                        .entry(candidate)
                                        .or_insert(HashSet::new())
                                        .insert(color);
                                } else {
                                    candidate_colors.entry(candidate).or_insert(HashSet::new());
                                }
                            }
                        }
                        if candidate_colors.iter().all(|(_, v)| v.len() != 1) {
                            continue;
                        }

                        // All colors are 1, but they must be the same color
                        if let Some((_, first_set)) = candidate_colors.iter().next() {
                            if first_set.len() == 1 {
                                let first_color = first_set.iter().next().unwrap();
                                if candidate_colors
                                    .iter()
                                    .all(|(_, set)| set.len() == 1 && set.contains(first_color))
                                {
                                    // All HashSet values are the same
                                    result = Some((uncolored_cell, *first_color));
                                    break;
                                } else {
                                    // There's at least one HashSet that's different
                                    continue;
                                }
                            }
                        }
                    }

                    result
                };

                if let Some((cell, color)) = conflicting_color {
                    visualizer_updates.push(VisualizerUpdate::SetTitle(
                        "3d Medusa: Cell Emptied by Color".to_string(),
                    ));
                    visualizer_updates.push(VisualizerUpdate::ColorCell(
                        cell.0,
                        cell.1,
                        Colors::CELL_MARKED_FOR_CANDIDATE_REMOVEAL,
                    ));
                    for (&(row, col, num), &c) in bicolored_graph.iter() {
                        if c == color {
                            visualizer_updates.push(VisualizerUpdate::ColorCandidate(
                                row,
                                col,
                                num,
                                Colors::CANDIDATE_MARKED_FOR_REMOVAL,
                            ));
                            reductions.push(SolverAction::CandidateReduction(row, col, num));
                        }
                    }
                    return Some((reductions, visualizer_updates));
                }
            }
        }

        None
    }
}
