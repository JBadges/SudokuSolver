extern crate sudoku_generator;

use sudoku_generator::sudoku_grid::*;
use sudoku_generator::solvers::sudoku_solver::*;

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
fn test_has_unique_solution_4() {
    assert!(SudokuGrid::from_string("300052000250300010004607523093200805570000030408035060005408300030506084840023056").has_unique_solution());
}
