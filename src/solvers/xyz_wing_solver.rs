use std::collections::HashSet;

use crate::sudoku_visualizer_builder::Colors;

use super::super::sudoku_grid::*;
use super::sudoku_solver::*;

use itertools::{iproduct, Itertools};

pub struct XYZWingSolver;

impl SudokuSolveMethod for XYZWingSolver {
    fn apply(&self, sgrid: &SudokuGrid) -> Option<SolverResult> {
        // Find all possible y-wing hinges and wings
        let cells_with_three_candidates: Vec<(usize, usize)> = iproduct!(0..9, 0..9)
            .filter(|&(row, col)| sgrid.candidates[row][col].len() == 3)
            .collect();
        let cells_with_two_candidates: Vec<(usize, usize)> = iproduct!(0..9, 0..9)
            .filter(|&(row, col)| sgrid.candidates[row][col].len() == 2)
            .collect();

        for &hinge in &cells_with_three_candidates {
            // Find all possible wings.
            // Possible wings are cells that can see the hinge that also have only 2 candidates.
            let hinge_candidates = &sgrid.candidates[hinge.0][hinge.1];
            let possible_wings: Vec<(usize, usize)> = cells_with_two_candidates
                .iter()
                .filter(|&&possible_wing| {
                    possible_wing != hinge
                        && SudokuGrid::cells_see_each_other(hinge, possible_wing)
                        && sgrid.candidates[possible_wing.0][possible_wing.1]
                            .iter()
                            .all(|&candidate| hinge_candidates.contains(&candidate))
                })
                .cloned()
                .collect();

            // For pairs of wings check if they work
            for wings in possible_wings.iter().cloned().combinations(2) {
                // The wings can't see each other to be valid
                if SudokuGrid::cells_see_each_other(wings[0], wings[1]) {
                    continue;
                }

                // Extract the candidates for the hinge and the two wings
                let wing1_candidates = &sgrid.candidates[wings[0].0][wings[0].1];
                let wing2_candidates = &sgrid.candidates[wings[1].0][wings[1].1];

                assert!(hinge_candidates.len() == 3);
                assert!(wing1_candidates.len() == 2);
                assert!(wing2_candidates.len() == 2);

                // Extract the three candidates from the hinge
                let hinge_values: Vec<usize> = hinge_candidates.iter().cloned().collect();

                // XYZ-Wing is form
                // hinge => XYZ
                // wing1 => YZ
                // wing2 => XZ

                // Find Z using both wings
                let z = if let Some(a) = hinge_values
                    .iter()
                    .find(|&&value| {
                        wing1_candidates.contains(&value) && wing2_candidates.contains(&value)
                    })
                    .cloned()
                {
                    a
                } else {
                    continue;
                };

                // Find Y using wing1
                let y = if let Some(a) = wing1_candidates.iter().find(|&&value| value != z).cloned()
                {
                    a
                } else {
                    continue;
                };

                // Find X using wing2
                let x = if let Some(a) = wing2_candidates.iter().find(|&&value| value != z).cloned()
                {
                    a
                } else {
                    continue;
                };

                // An XYZ wing must have all distinct values
                if x == y || x == z || y == z {
                    continue;
                }

                assert!(hinge_candidates.contains(&x));
                assert!(hinge_candidates.contains(&y));
                assert!(hinge_candidates.contains(&z));

                assert!(wing1_candidates.contains(&y));
                assert!(wing1_candidates.contains(&z));

                assert!(wing2_candidates.contains(&x));
                assert!(wing2_candidates.contains(&z));

                // We can remove the shared candidate between the wings
                // in all cells where the wings intersect
                let cells_seen_from_hinge = SudokuGrid::generate_cells_seen_from_cord(hinge);
                let cells_seen_from_wing1 = SudokuGrid::generate_cells_seen_from_cord(wings[0]);
                let cells_seen_from_wing2 = SudokuGrid::generate_cells_seen_from_cord(wings[1]);

                let shared_cells = cells_seen_from_hinge
                    .intersection(&cells_seen_from_wing1)
                    .cloned()
                    .collect::<HashSet<_>>()
                    .intersection(&cells_seen_from_wing2)
                    .cloned()
                    .collect::<HashSet<_>>();

                let mut visualizer_updates = Vec::new();
                let mut reductions = Vec::new();
                visualizer_updates.push(VisualizerUpdate::SetTitle("XYZ-Wing".to_string()));

                for (row, col) in [hinge, wings[0], wings[1]] {
                    visualizer_updates.push(VisualizerUpdate::ColorCell(
                        row,
                        col,
                        Colors::CELL_USED_TO_DETERMINE_SOLUTION,
                    ));
                    for &candidate in &sgrid.candidates[row][col] {
                        visualizer_updates.push(VisualizerUpdate::ColorCandidate(
                            row,
                            col,
                            candidate,
                            Colors::DIGIT_USED_TO_DETERMINE_SOLUTION,
                        ));
                    }
                }

                for cell in shared_cells {
                    if cell == hinge || cell == wings[0] || cell == wings[1] {
                        continue;
                    }
                    let (row, col) = cell;
                    visualizer_updates.push(VisualizerUpdate::ColorCell(
                        row,
                        col,
                        Colors::CELL_MARKED_FOR_CANDIDATE_REMOVEAL,
                    ));
                    if sgrid.candidates[row][col].contains(&z) {
                        reductions.push(SolverAction::CandidateReduction(row, col, z));
                        visualizer_updates.push(VisualizerUpdate::ColorCandidate(
                            row,
                            col,
                            z,
                            Colors::CANDIDATE_MARKED_FOR_REMOVAL,
                        ));
                    }
                }
                if !reductions.is_empty() {
                    return Some((reductions, visualizer_updates));
                }
            }
        }

        None
    }
}
