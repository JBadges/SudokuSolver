extern crate sudoku_generator;

use sudoku_generator::sudoku_grid::*;

#[test]
fn test_backtrack_fill() {
    assert!(SudokuGrid::from_string("783294156006813729912000483090000000030070000820309600008601000160930008009085061").backtrack_fill());
}

#[test]
fn test_has_unique_solution_1() {
    assert!(SudokuGrid::from_string("783294156006813729912000483090000000030070000820309600008601000160930008009085061").has_unique_solution());
}

#[test]
fn test_has_unique_solution_2() {
    assert!(SudokuGrid::from_string("000000000001900500560310090100600028004000700270004003040068035002005900000000000").has_unique_solution());
}

#[test]
fn test_has_unique_solution_3() {
    assert!(SudokuGrid::from_string("010800907060239108908000263096300082830070019501900034000600471600000825180000396").has_unique_solution());
}

#[test]
fn test_3dmedusa_rule1() {
    // TODO(JBadges): Solver testing framework
    assert!(SudokuGrid::from_string("093824560085600002206075008321769845000258300578040296850016723007082650002507180").has_unique_solution());
}

#[test]
fn test_cells_contained_in_only_box() {
    let contained = SudokuGrid::get_contained_units(&vec![(1,0),(1,1),(2,0),(2,1)]);
    assert!(contained.len() == 1);
    assert!(contained[0] == UnitType::Box);
}

#[test]
fn test_cells_contained_empty() {
    let contained = SudokuGrid::get_contained_units(&vec![]);
    assert!(contained.len() == 0);
}
