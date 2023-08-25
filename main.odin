package main

import "core:fmt"
import "core:math/rand"

SudokuGrid :: struct {
    // [row][col]
    grid: [9][9]int
}

display_grid :: proc(sgrid: ^SudokuGrid) {
    for row in 0..<9 {
        if row % 3 == 0 && row != 0 {
            fmt.println("- - - - -+- - - - -+- - - - -")  // Print horizontal separator every 3 rows
        }
        
        for col in 0..<9 {
            if col % 3 == 0 && col != 0 {
                fmt.print("| ")  // Print vertical separator every 3 columns
            }
            
            // Print the number with two spaces after it
            if sgrid.grid[row][col] == 0 {
                fmt.print(".  ")  // Use a dot for empty cells followed by two spaces
            } else {
                fmt.print(sgrid.grid[row][col], " ")
            }
            
            if col == 8 {
                fmt.println()  // Move to the next line after the last column
            }
        }
    }
}



create_sudoku_grid_randomly :: proc(cells_to_fill: int) -> SudokuGrid {
    my_rand: rand.Rand
    rand.init(&my_rand, 1)
    for true {
        sgrid: SudokuGrid
        for _ in 0..<cells_to_fill {
            row, col, num := rand.int_max(9, &my_rand), rand.int_max(9, &my_rand), rand.int_max(9, &my_rand)
            for sgrid.grid[row][col] != 0 || !is_valid_sudoku_placement(&sgrid, num, row, col) {
                row, col, num = rand.int_max(9, &my_rand), rand.int_max(9, &my_rand), rand.int_max(9, &my_rand)
            }
            sgrid.grid[row][col] = num
        }
        if solve_grid(&sgrid) { return sgrid }
    }

    return {}
}

solve_grid :: proc(sgrid: ^SudokuGrid) -> bool {
    for row in 0..<9 {
        for col in 0..<9 {
            if sgrid.grid[row][col] == 0 {
                for num in 1..=9 {
                    if is_valid_sudoku_placement(sgrid, num, row, col) {
                        sgrid.grid[row][col] = num
                        if solve_grid(sgrid) { return true }
                        sgrid.grid[row][col] = 0
                    }
                }
                return false
            }
        }
    }
    return true
}

is_valid_sudoku_placement :: proc(sgrid: ^SudokuGrid, digit: int, row: int, col: int) -> bool {
    // Row/Col checks
    for i in 0..<9 {
        if sgrid.grid[row][i] == digit { return false }
        if sgrid.grid[i][col] == digit { return false }
    }

    // Box checks
    start_row, start_col := 3 * (row / 3), 3 * (col / 3)
    for i in 0..<3 {
        for j in 0..<3 {
            if sgrid.grid[i + start_row][j + start_col] == digit { return false }
        }
    }

    return true
}

main :: proc() {
    sg: SudokuGrid
    display_grid(&sg)
    sg = create_sudoku_grid_randomly(30)
    display_grid(&sg)
}