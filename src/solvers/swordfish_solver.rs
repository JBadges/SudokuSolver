use super::sudoku_solver::*;
use super::super::sudoku_grid::*;

use std::collections::HashSet;
use itertools::Itertools;
use crate::sudoku_latex_builder::SudokuLatexBuilder;

pub struct SwordfishSolver;

impl SudokuSolveMethod for SwordfishSolver {
    fn apply(&self, sgrid: &mut SudokuGrid, visualizer: &mut SudokuLatexBuilder) -> bool {

        let mut applied = false;

        for num in 1..=9 {
            fn swordfish_for_direction(sgrid: &mut SudokuGrid, num: usize, is_rowwise: bool) -> bool {
                let mut applied = false;
                let mut candidates: [HashSet<usize>; 9] = Default::default();
                
                for primary in 0..9 {
                    for secondary in 0..9 {
                        if is_rowwise && sgrid.candidates[primary][secondary].contains(&num) {
                            candidates[primary].insert(secondary);
                        } else if !is_rowwise && sgrid.candidates[secondary][primary].contains(&num) {
                            candidates[primary].insert(secondary);
                        }
                    }
                }

                let candidate_count = candidates
                    .iter()
                    .filter(|x| x.len() == 2 || x.len() == 3)
                    .count();
                if candidate_count < 3 { return false; }

                for primary_indices in (0..9).combinations(3) {
                    let i = primary_indices[0];
                    let j = primary_indices[1];
                    let k = primary_indices[2];

                    let unioned_indices = candidates[i]
                        .union(&candidates[j])
                        .copied()
                        .collect::<HashSet<usize>>()
                        .union(&candidates[k])
                        .cloned()
                        .collect::<HashSet<usize>>();

                    if unioned_indices.len() != 3 { continue; }

                    // We found a swordfish, now find reduction candidates
                    for secondary in &unioned_indices {
                        for primary in 0..9 {
                            if primary == i || primary == j || primary == k { continue; }
                            if is_rowwise && sgrid.candidates[primary][*secondary].remove(&num) {
                                applied = true;
                                println!(
                                    "Solver [SwordfishSolver:RowSF->ColRem] removed value {} from candidate location ({}, {}). SF {:?} {:?}",
                                    num, primary, secondary, &primary_indices, &unioned_indices
                                );
                            } else if !is_rowwise && sgrid.candidates[*secondary][primary].remove(&num) {
                                applied = true;
                                println!(
                                    "Solver [SwordfishSolver:ColSF->RowRem] removed value {} from candidate location ({}, {}). SF {:?} {:?}",
                                    num, secondary, primary, &primary_indices, &unioned_indices
                                );
                            }
                        }
                    }
                }
                applied
            }

            applied |= swordfish_for_direction(sgrid, num, true);  // Row-wise
            applied |= swordfish_for_direction(sgrid, num, false); // Column-wise
        }

        applied
    }
}
