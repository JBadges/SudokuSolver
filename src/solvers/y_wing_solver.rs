use crate::sudoku_visualizer_builder::Colors;

use super::sudoku_solver::*;
use super::super::sudoku_grid::*;

use itertools::{Itertools, iproduct};

pub struct YWingSolver;

impl SudokuSolveMethod for YWingSolver {
    fn apply(&self, sgrid: &SudokuGrid) -> Option<SolverResult> {

        // Find all possible y-wing hinges and wings
        let cells_with_two_candidates: Vec<(usize, usize)> = iproduct!(0..9, 0..9)
            .filter(|&(row, col)| sgrid.candidates[row][col].len() == 2)
            .collect();

        for &hinge in &cells_with_two_candidates {
            // Find all possible wings. 
            // Possible wings are cells that can see the hinge that also have only 2 candidates.
            let possible_wings: Vec<(usize, usize)> = cells_with_two_candidates.iter()
                .filter(|&&possible_wing| possible_wing != hinge && SudokuGrid::cells_see_each_other(hinge, possible_wing))
                .cloned()
                .collect();


            // For pairs of wings check if they work
            for wings in possible_wings.iter().cloned().combinations(2) {
                // The wings can't see each other to be valid
                if SudokuGrid::cells_see_each_other(wings[0], wings[1]) { continue; }

                // Extract the candidates for the hinge and the two wings
                let hinge_candidates = &sgrid.candidates[hinge.0][hinge.1];
                let wing1_candidates = &sgrid.candidates[wings[0].0][wings[0].1];
                let wing2_candidates = &sgrid.candidates[wings[1].0][wings[1].1];
                
                assert!(hinge_candidates.len() == 2);
                assert!(wing1_candidates.len() == 2);
                assert!(wing2_candidates.len() == 2);

                // Extract the two candidates from the hinge
                let hinge_values: Vec<usize> = hinge_candidates.iter().cloned().collect();

                // Y-Wing is form 
                // hinge => AB
                // wing1 => AC 
                // wing2 => BC

                // Find A & B using wing1
                let (a, b) = if wing1_candidates.contains(&hinge_values[0]) {
                    (hinge_values[0], hinge_values[1])
                } else if wing1_candidates.contains(&hinge_values[1]) {
                    (hinge_values[1], hinge_values[0])
                } else {
                    continue;
                };

                // Get C from wing1
                let c_from_wing1 = wing1_candidates.iter().find(|&&x| x != a).cloned();

                // Get C from wing2
                if !wing2_candidates.contains(&b) { continue; }
                let c_from_wing2 = wing2_candidates.iter().filter(|&&x| x != b).cloned().next();


                let c = if let (Some(c1), Some(c2)) = (c_from_wing1, c_from_wing2) {
                    if c1 == c2 {
                        c1
                    } else {
                        continue;
                    }
                } else {
                    continue;
                };

                assert!(hinge_candidates.contains(&a));
                assert!(hinge_candidates.contains(&b));

                assert!(wing1_candidates.contains(&a));
                assert!(wing1_candidates.contains(&c));

                assert!(wing2_candidates.contains(&b));
                assert!(wing2_candidates.contains(&c));

                // We can remove the shared candidate between the wings
                // in all cells where the wings intersect
                let cells_seen_from_wing1 = SudokuGrid::generate_cells_seen_from_cord(wings[0]);
                let cells_seen_from_wing2 = SudokuGrid::generate_cells_seen_from_cord(wings[1]);


                let shared_cells = cells_seen_from_wing1.intersection(&cells_seen_from_wing2);
                
                let mut visualizer_updates = Vec::new();
                let mut reductions = Vec::new();
                visualizer_updates.push(VisualizerUpdate::SetTitle("Y-Wing".to_string()));

                for (row, col) in [hinge, wings[0], wings[1]] {
                    visualizer_updates.push(VisualizerUpdate::ColorCell(row, col, Colors::CELL_USED_TO_DETERMINE_SOLUTION));
                    for &candidate in &sgrid.candidates[row][col] {
                        visualizer_updates.push(VisualizerUpdate::ColorCandidate(row, col, candidate, Colors::DIGIT_USED_TO_DETERMINE_SOLUTION));
                    }
                }

                for &cell in shared_cells {
                    if cell == hinge || cell == wings[0] || cell == wings[1] { continue; }
                    let (row, col) = cell;
                    visualizer_updates.push(VisualizerUpdate::ColorCell(row, col, Colors::CELL_MARKED_FOR_CANDIDATE_REMOVEAL));
                    if sgrid.candidates[row][col].contains(&c) {
                        reductions.push(SolverAction::CandidateReduction(row, col, c));
                        visualizer_updates.push(VisualizerUpdate::ColorCandidate(row, col, c, Colors::CANDIDATE_MARKED_FOR_REMOVAL));
                    }
                }
                if !reductions.is_empty() { return Some((reductions, visualizer_updates)); }
            }
        }

        None
    }
}
