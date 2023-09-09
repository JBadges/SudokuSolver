use raylib::prelude::*;

use sudoku_generator::solvers::solver_manager::SudokuSolverManager;
use sudoku_generator::sudoku_grid::*;
use sudoku_generator::solvers::single_candidate_solver::SingleCandidateSolver;
use sudoku_generator::solvers::naked_singles_solver::NakedSinglesSolver;
use sudoku_generator::solvers::naked_candidates_solver::NakedCandidatesSolver;
use sudoku_generator::solvers::hidden_candidates_solver::HiddenCandidatesSolver;
use sudoku_generator::solvers::intersection_removal_solver::IntersectionRemovalSolver;
use sudoku_generator::solvers::x_wing_solver::XWingSolver;
use sudoku_generator::solvers::singles_chains_solver::SinglesChainsSolver;
use sudoku_generator::solvers::y_wing_solver::YWingSolver;
use sudoku_generator::solvers::swordfish_solver::SwordfishSolver;
use sudoku_generator::solvers::jellyfish_solver::JellyfishSolver;
use sudoku_generator::solvers::medusa_3d_solver::Medusa3DSolver;
use sudoku_generator::solvers::bowmans_bingo_solver::BowmansBingoSolver;

fn draw_text_centered(d: &mut RaylibDrawHandle, text: &str, cell_center_x: i32, cell_center_y: i32, text_size: i32, color: Color) {
    let text_width = measure_text(text, text_size);
    let text_height = text_size;  // Approximation, should be fine for digits
    let x = cell_center_x - text_width / 2;
    let y = cell_center_y - text_height / 2;
    d.draw_text(text, x, y, text_size, color);
}

fn main() {
    const GRID_TOP_LEFT_X: i32 = 50;
    const GRID_TOP_LEFT_Y: i32 = 100;
    const CELL_SIZE: i32 = 80;
    const GRID_SIZE: i32 = 9;
    const MAJOR_LINE_WIDTH: f32 = 4.0;
    const MINOR_LINE_WIDTH: f32 = 1.0;

    let (mut rl, thread) = raylib::init()
    .size(800, 900)
    .title("Sudoku Visualizer")
    .build();

    // let rand_grid = SudokuGrid::create_sudoku_puzzle(100);
    // let hidden_single = SudokuGrid::from_string("720096003000205000080004020000000060106503807040000000030800090000702000200430018");
    // let hidden_triple = SudokuGrid::from_string("300000000970010000600583000200000900500621003008000005000435002000090056000000001");
    // let simplest_sudoku = SudokuGrid::from_string("000105000140000670080002400063070010900000003010090520007200080026000035000409000");
    // let intersection_removal = SudokuGrid::from_string("000921003009000060000000500080403006007000800500700040003000000020000700800195000");
    // let xwing = SudokuGrid::from_string("093004560060003140004608309981345000347286951652070483406002890000400010029800034");
    // let simple_col_2 = SudokuGrid::from_string("123000587005817239987000164051008473390750618708100925076000891530081746810070352");
    // let simple_col_4 = SudokuGrid::from_string("036210840800045631014863009287030456693584000145672398408396000350028064060450083");
    // let swordfish = SudokuGrid::from_string("050030602642895317037020800023504700406000520571962483214000900760109234300240170");
    // let jellyfish = SudokuGrid::from_string("024090008800402900719000240075804300240900587038507604082000059007209003490050000");
    // let medusa_twice_in_a_cell = SudokuGrid::from_string("093824560085600002206075008321769845000258300578040296850016723007082650002507180");
    let medusa_twice_in_a_unit = SudokuGrid::from_string("300052000250300010004607523093200805570000030408035060005408300030506084840023056");
    // let medusa_two_colors_in_a_cell = SudokuGrid::from_string("290000830000020970000109402845761293600000547009045008903407000060030709050000384");
    // let medusa_two_colours_elsewhere = SudokuGrid::from_string("100056003043090000800043002030560210950421037021030000317980005000310970000670301");
    // let medusa_cell_emptied_by_color = SudokuGrid::from_string("986721345304956007007030960073065009690017003100390276000679030069143700731582694");
    
    let grid = medusa_twice_in_a_unit;

    let mut solver: SudokuSolverManager = SudokuSolverManager::new(grid.clone());
    println!("Sudoku id: {}", grid.to_number_string());

    solver.add_solver(Box::new(SingleCandidateSolver));
    solver.add_solver(Box::new(NakedSinglesSolver));
    solver.add_solver(Box::new(NakedCandidatesSolver));
    solver.add_solver(Box::new(HiddenCandidatesSolver));
    solver.add_solver(Box::new(IntersectionRemovalSolver));
    solver.add_solver(Box::new(XWingSolver));
    solver.add_solver(Box::new(SinglesChainsSolver));
    solver.add_solver(Box::new(YWingSolver));
    solver.add_solver(Box::new(SwordfishSolver));
    solver.add_solver(Box::new(JellyfishSolver));
    solver.add_solver(Box::new(Medusa3DSolver));
    solver.add_solver(Box::new(BowmansBingoSolver));

    let mut iter = 0;
    let mut done = false;

    solver.solve_iteration();
    while !rl.window_should_close() {
        let builder = &solver.visualizers_per_step.last().unwrap()[iter];
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
    
        // Draw title
        draw_text_centered(&mut d, &builder.title, 400, 25, 30, Color::BLACK);
    
        // Highlights
        for ((row, col), color) in &builder.cell_highlights {
            let x = GRID_TOP_LEFT_X + *col as i32 * CELL_SIZE;
            let y = GRID_TOP_LEFT_Y + *row as i32 * CELL_SIZE;
            d.draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, color);
        }

        for ((row, col), color) in &builder.digits_highlights {
            let x = GRID_TOP_LEFT_X + *col as i32 * CELL_SIZE;
            let y = GRID_TOP_LEFT_Y + *row as i32 * CELL_SIZE;
            d.draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, color);
        }

        for ((row, col, num), color) in &builder.candidates_highlights {
            let x_offset = (((*num as i32 - 1) % 3 - 1) * 20) as i32;
            let y_offset = (((*num as i32 - 1) / 3 - 1) * 20) as i32;
            
            let cell_x = GRID_TOP_LEFT_X + *col as i32 * CELL_SIZE + CELL_SIZE / 2 + x_offset - 5;
            let cell_y = GRID_TOP_LEFT_Y + *row as i32 * CELL_SIZE + CELL_SIZE / 2 + y_offset - 5;
            
            d.draw_rectangle(cell_x, cell_y, 10, 10, color);
        }

        // Draw grid
        for i in 0..=GRID_SIZE {
            let x = GRID_TOP_LEFT_X + i * CELL_SIZE;
            let y = GRID_TOP_LEFT_Y + i * CELL_SIZE;
            
            // Determine line width based on whether this is a major line
            let line_width = if i % 3 == 0 { MAJOR_LINE_WIDTH } else { MINOR_LINE_WIDTH };

            // Draw horizontal line
            d.draw_line_ex(
                Vector2::new(x as f32, GRID_TOP_LEFT_Y as f32),
                Vector2::new(x as f32, (GRID_TOP_LEFT_Y + GRID_SIZE * CELL_SIZE) as f32),
                line_width,
                Color::BLACK,
            );

            // Draw vertical line
            d.draw_line_ex(
                Vector2::new(GRID_TOP_LEFT_X as f32, y as f32),
                Vector2::new((GRID_TOP_LEFT_X + GRID_SIZE * CELL_SIZE) as f32, y as f32),
                line_width,
                Color::BLACK,
            );
        }
    
        // Draw row and column names
        for (i, name) in ('A'..='I').enumerate() {
            let x = GRID_TOP_LEFT_X - 20;  // 20 units to the left of the grid
            let y = GRID_TOP_LEFT_Y + i as i32 * CELL_SIZE + CELL_SIZE / 2;
            draw_text_centered(&mut d, &name.to_string(), x, y, 20, Color::BLACK);
        }


        for i in 1..=GRID_SIZE {
            let x = GRID_TOP_LEFT_X + i as i32 * CELL_SIZE - CELL_SIZE / 2;
            let y = GRID_TOP_LEFT_Y - 20;  // 20 units above the grid
            draw_text_centered(&mut d, &i.to_string(), x, y, 20, Color::BLACK);
        }

        // Draw digits
        for (&(row, col), &(num, color)) in &builder.digits {
            let cell_center_x = GRID_TOP_LEFT_X + col as i32 * CELL_SIZE + CELL_SIZE / 2;
            let cell_center_y = GRID_TOP_LEFT_Y + row as i32 * CELL_SIZE + CELL_SIZE / 2;
            draw_text_centered(&mut d, &num.to_string(), cell_center_x, cell_center_y, 40, color);
        }

        // Draw candidates
        for (&(row, col, num), &color) in &builder.candidates {
            let x_offset = (((num as i32 - 1) % 3 - 1) * 20) as i32;
            let y_offset = (((num as i32 - 1) / 3 - 1) * 20) as i32;
            
            let cell_center_x = GRID_TOP_LEFT_X + col as i32 * CELL_SIZE + CELL_SIZE / 2 + x_offset;
            let cell_center_y = GRID_TOP_LEFT_Y + row as i32 * CELL_SIZE + CELL_SIZE / 2 + y_offset;
            
            draw_text_centered(&mut d, &num.to_string(), cell_center_x, cell_center_y, 10, color);
        }
    
        // Draw chains
        for (&((row_from, col_from, num_from), (row_to, col_to, num_to)), color) in &builder.chains {
            let x_offset_from = (((num_from as i32 - 1) % 3 - 1) * 20) as i32;
            let y_offset_from = (((num_from as i32 - 1) / 3 - 1) * 20) as i32;
            let x_offset_to = (((num_to as i32 - 1) % 3 - 1) * 20) as i32;
            let y_offset_to = (((num_to as i32 - 1) / 3 - 1) * 20) as i32;
        
            let x1 = GRID_TOP_LEFT_X + col_from as i32 * CELL_SIZE + CELL_SIZE / 2 + x_offset_from;
            let y1 = GRID_TOP_LEFT_Y + row_from as i32 * CELL_SIZE + CELL_SIZE / 2 + y_offset_from;
            let x2 = GRID_TOP_LEFT_X + col_to as i32 * CELL_SIZE + CELL_SIZE / 2 + x_offset_to;
            let y2 = GRID_TOP_LEFT_Y + row_to as i32 * CELL_SIZE + CELL_SIZE / 2 + y_offset_to;
        
            let control_point = Vector2::new((x1 + x2) as f32 / 2.0 - 25.0, (y1 + y2) as f32 / 2.0 - 25.0);
            let start_point = Vector2::new(x1 as f32, y1 as f32);
            let end_point = Vector2::new(x2 as f32, y2 as f32);
        
            d.draw_line_bezier_quad(start_point, end_point, control_point, 2.0, color);
        }
        
        if d.is_key_pressed(KeyboardKey::KEY_SPACE) && !done {
            if iter == 2 {
                println!("Running next solver iteration");
                if !solver.solve_iteration() {
                    done = true;
                }
                iter = 0;
            } else {
                println!("Iterating to next debug stage");
                iter += 1;
            }
        }
    }




//     println!("{}", grid);
//     println!("{}", grid.to_number_string());
//     println!("Generated puzzle with {} blanks", grid.grid.iter().flatten().filter(|&&x| x == 0).count());
    


//     let mut applied = true;
//     while applied {
//         applied = solver.solve_iteration();
//     }

//     solver.generate_output_pdf("solver.pdf");

//     println!("Unable to apply more solvers. Final board state:");
//     println!("{}", solver.sgrid);
}
