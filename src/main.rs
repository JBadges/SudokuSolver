use std::collections::HashSet;

use raylib::prelude::*;
use sudoku_generator::solvers::solver_manager::SudokuSolverManager;
use sudoku_generator::solvers::x_chain_solver::XChainSolver;
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
use sudoku_generator::solvers::xyz_wing_solver::XYZWingSolver;
use sudoku_generator::solvers::medusa_3d_solver::Medusa3DSolver;
use sudoku_generator::solvers::bowmans_bingo_solver::BowmansBingoSolver;
use sudoku_generator::sudoku_visualizer_builder::SudokuVisualizerBuilder;

fn draw_text_centered(d: &mut RaylibDrawHandle, text: &str, cell_center_x: i32, cell_center_y: i32, text_size: i32, color: Color) {
    let text_width = measure_text(text, text_size);
    let text_height = text_size;  // Approximation, should be fine for digits
    let x = cell_center_x - text_width / 2;
    let y = cell_center_y - text_height / 2;
    d.draw_text(text, x, y, text_size, color);
}


fn draw_sgrid(canvas_offset_x: i32, canvas_offset_y: i32, canvas_width: i32, canvas_height: i32, draw: &mut RaylibDrawHandle<'_>, builder: &SudokuVisualizerBuilder, sgrid: &SudokuGrid) {
    let size = std::cmp::min(canvas_width, canvas_height) as f32;
    let offset_x = canvas_offset_x + ((canvas_width as f32 - size) / 2.0) as i32;
    let offset_y = canvas_offset_y + ((canvas_height as f32 - size) / 2.0) as i32;

    const BORDER_USAGE: f32 = 0.05;
    const LINE_SPACING: f32 = (1.0 - 2.0 * BORDER_USAGE) / 9.0; // 9 segments
    let line_thickness: f32 = 2.0;
    let chain_thickness: f32 = 2.0;
    let candidate_text_size = 10;

    let get_xy_for_candidate = |row: usize, col: usize, num: usize| -> (f32, f32) {
        let x_offset = ((num as i32 - 1) % 3 - 1) as f32 * LINE_SPACING * size / 4.0; // 4.0 is a random number that looks nice in the grid.
        let y_offset = ((num as i32 - 1) / 3 - 1) as f32 * LINE_SPACING * size / 4.0;
        
        let cell_center_x = offset_x as f32 + (col as f32 + 1.0) * LINE_SPACING * size + x_offset;
        let cell_center_y = offset_y as f32 + (row as f32 + 1.0) * LINE_SPACING * size + y_offset;
    
        (cell_center_x, cell_center_y)
    };

    // Highlights - Cell
    for (&(row, col), &color) in &builder.cell_highlights {
        let x = offset_x as f32 + (col + 1) as f32 * LINE_SPACING * size - LINE_SPACING * size / 2.0 + line_thickness / 2.0;
        let y = offset_y as f32 + (row + 1) as f32 * LINE_SPACING * size - LINE_SPACING * size / 2.0 + line_thickness / 2.0;
        draw.draw_rectangle(x as i32, y as i32, (LINE_SPACING * size) as i32, (LINE_SPACING * size) as i32, color);
    }

    // Highlights - Digit
    for (&(row, col, num), &color) in &builder.candidates_highlights {
        let (x, y) = get_xy_for_candidate(row, col, num);

        draw.draw_rectangle(x as i32 - candidate_text_size / 2, y as i32 - candidate_text_size / 2, candidate_text_size, candidate_text_size, color);
    }

    for line in [1,2,4,5,7,8,0,3,6,9] { 
        let color = if line % 3 == 0 {
            Color::BLACK
        } else {
            Color::LIGHTGRAY
        };

        let offset = BORDER_USAGE * size + line as f32 * LINE_SPACING * size;

        // Draw horizontal lines
        draw.draw_line_ex(
            Vector2 { x: (BORDER_USAGE * size + offset_x as f32), y: (offset + offset_y as f32) }, 
            Vector2 { x: ((1.0 - BORDER_USAGE) * size + offset_x as f32), y: (offset + offset_y as f32) }, 
            line_thickness,
            color
        );

        // Draw vertical lines
        draw.draw_line_ex(
            Vector2 { x: (offset + offset_x as f32), y: (BORDER_USAGE * size + offset_y as f32) },
            Vector2 { x: (offset + offset_x as f32), y: ((1.0 - BORDER_USAGE) * size + offset_y as f32) },
            line_thickness,
            color
        );
    }

    // Draw row and column names
    for (i, name) in ('A'..='I').enumerate() {
        let x_left = offset_x as f32 + (BORDER_USAGE * size) / 2.0;
        let x_right = offset_x as f32 + (1.0 - BORDER_USAGE) * size + (BORDER_USAGE * size) / 2.0;
        let y = offset_y as f32 + (i as f32 + 1.0) * LINE_SPACING * size;
        draw_text_centered(draw, &name.to_string(), x_left as i32, y as i32, 20, Color::BLACK);
        draw_text_centered(draw, &name.to_string(), x_right as i32, y as i32, 20, Color::BLACK);
    }


    for i in 1..=9 {
        let x = offset_x as f32 + i as f32 * LINE_SPACING * size;
        let y_top = offset_y as f32 + (BORDER_USAGE * size) / 2.0;
        let y_bot = offset_y as f32 + (1.0 - BORDER_USAGE) * size + (BORDER_USAGE * size) / 2.0;
        draw_text_centered(draw, &i.to_string(), x as i32, y_top as i32, 20, Color::BLACK);
        draw_text_centered(draw, &i.to_string(), x as i32, y_bot as i32, 20, Color::BLACK);
    }

    // Draw digits
    for (&(row, col), &(num, color)) in &builder.digits {
        let cell_center_x = offset_x as f32 + (col as f32 + 1.0) * LINE_SPACING * size;
        let cell_center_y = offset_y as f32 + (row as f32 + 1.0) * LINE_SPACING * size;
        draw_text_centered(draw, &num.to_string(), cell_center_x as i32, cell_center_y as i32, 40, color);
    }

    let mut drawn_pairs: HashSet<((usize, usize, usize), (usize, usize, usize))> = Default::default();
    for (&((row_from, col_from, num_from), (row_to, col_to, num_to)), &color) in &builder.chains {
        let current_pair = ((row_from, col_from, num_from), (row_to, col_to, num_to));
        let reversed_pair = ((row_to, col_to, num_to), (row_from, col_from, num_from));
    
        if drawn_pairs.contains(&reversed_pair) {
            continue;
        }
    
        drawn_pairs.insert(current_pair);
    
        let a = {
            let (x, y) = get_xy_for_candidate(row_from, col_from, num_from);
            Vector2 { x, y }
        };
    
        let b = {
            let (x, y) = get_xy_for_candidate(row_to, col_to, num_to);
            Vector2 { x, y }
        };
    
        fn is_point_on_line_segment(a: &Vector2, b: &Vector2, point: &Vector2) -> bool {
            let cross_product = (point.y - a.y) * (b.x - a.x) - (point.x - a.x) * (b.y - a.y);
            if cross_product.abs() > f32::EPSILON {
                return false;
            }
        
            let dot_product = (point.x - a.x) * (b.x - a.x) + (point.y - a.y) * (b.y - a.y);
            if dot_product < 0.0 {
                return false;
            }
        
            let squared_length = (b.x - a.x) * (b.x - a.x) + (b.y - a.y) * (b.y - a.y);
            if dot_product > squared_length {
                return false;
            }
        
            true
        }

        let intersects = (0..9).flat_map(|row| {
            (0..9).flat_map(move |col| {
                sgrid.candidates[row][col].iter().filter_map(move |&num| {
                    if sgrid.grid[row][col] != 0 {
                        return None;
                    }
                    let point = {
                        let (x, y) = get_xy_for_candidate(row, col, num);
                        Vector2 { x, y }
                    };
    
                    if (row, col, num) != (row_from, col_from, num_from) && (row, col, num) != (row_to, col_to, num_to) && is_point_on_line_segment(&a, &b, &point) {
                        Some(point)
                    } else {
                        None
                    }
                })
            })
        }).next().is_some();

        if !intersects {
            draw.draw_line_ex(a, b, chain_thickness, color);
            continue;
        }
    
        let m = (a + b) / 2.0;       
    
        let perp = Vector2 {
            x: - (a.y - b.y),
            y: a.x - b.x,
        };
    
        let length = perp.length();
        let perp_normalized = Vector2 {
            x: perp.x / length,
            y: perp.y / length,
        };
    
        let distance = ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt();
        let offset = 0.07 * size * (distance / size).min(1.0);
    
        let control_point = Vector2 {
            x: m.x + perp_normalized.x * offset,
            y: m.y + perp_normalized.y * offset,
        };
    
        draw.draw_line_bezier_quad(a, b, control_point, chain_thickness, color);
    }

    // Draw candidates
    for (&(row, col, num), &color) in &builder.candidates {
        let (cell_center_x, cell_center_y) = get_xy_for_candidate(row, col, num);
        
        draw_text_centered(draw, &num.to_string(), cell_center_x as i32, cell_center_y as i32, candidate_text_size, color);
    }
}


fn main() {
    #[cfg(feature = "profiling")]
    const PROFILING: bool = true;
    #[cfg(not(feature = "profiling"))]
    const PROFILING: bool = false;

    const HEADER_SPACING: f32 = 0.05;

    let (mut rl, thread) = raylib::init()
        .size(800, 800)
        .title("Sudoku Visualizer")
        .resizable()
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
    // let medusa_twice_in_a_unit = SudokuGrid::from_string("300052000250300010004607523093200805570000030408035060005408300030506084840023056");
    // let medusa_two_colors_in_a_cell = SudokuGrid::from_string("290000830000020970000109402845761293600000547009045008903407000060030709050000384");
    // let medusa_two_colours_elsewhere = SudokuGrid::from_string("100056003043090000800043002030560210950421037021030000317980005000310970000670301");
    // let medusa_cell_emptied_by_color = SudokuGrid::from_string("986721345304956007007030960073065009690017003100390276000679030069143700731582694");
    // let xyz_wing = SudokuGrid::from_string("092001750500200008000030200075004960200060075069700030008090020700003089903800040");
    // let x_chain_1 = SudokuGrid::from_string("000351780857629341100874002509162804681040200000008160718000020000010008060080017");
    let x_chain_2 = SudokuGrid::from_string("700054010063870425504000700270400001400920007000007542852043079390782054047590283");
    let grid = x_chain_2;

    let mut solver: SudokuSolverManager = SudokuSolverManager::new(grid.clone());
    println!("Sudoku id: {}", grid.to_number_string());

    solver.add_solver(Box::new(SingleCandidateSolver));
    solver.add_solver(Box::new(NakedSinglesSolver));
    solver.add_solver(Box::new(NakedCandidatesSolver::<2>));
    solver.add_solver(Box::new(NakedCandidatesSolver::<3>));
    solver.add_solver(Box::new(HiddenCandidatesSolver::<2>));
    solver.add_solver(Box::new(HiddenCandidatesSolver::<3>));
    solver.add_solver(Box::new(NakedCandidatesSolver::<4>));
    solver.add_solver(Box::new(HiddenCandidatesSolver::<4>));
    solver.add_solver(Box::new(IntersectionRemovalSolver));
    solver.add_solver(Box::new(XWingSolver));
    solver.add_solver(Box::new(XChainSolver));
    solver.add_solver(Box::new(SinglesChainsSolver));
    solver.add_solver(Box::new(YWingSolver));
    solver.add_solver(Box::new(SwordfishSolver::<3>));
    solver.add_solver(Box::new(XYZWingSolver));
    solver.add_solver(Box::new(SwordfishSolver::<4>));
    solver.add_solver(Box::new(Medusa3DSolver));
    solver.add_solver(Box::new(BowmansBingoSolver));

    let mut iter = 0;
    let mut done = false;

    solver.solve_iteration();
    while !rl.window_should_close() {
        let builder = &solver.visualizers_per_step.last().unwrap()[iter];
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        let screen_width = d.get_screen_width();
        let screen_height = d.get_screen_height();
        let header_pixles = HEADER_SPACING * screen_height as f32;
        // Draw title
        draw_text_centered(&mut d, &builder.title, screen_width / 2, (header_pixles / 2.0) as i32, (0.8 * header_pixles) as i32, Color::BLACK);
    
        draw_sgrid(0,header_pixles as i32, screen_width, (screen_height as f32 - header_pixles) as i32, &mut d, builder, &solver.sgrid);
        
        if PROFILING && done { 
            break;
        }

        if PROFILING || (d.is_key_pressed(KeyboardKey::KEY_SPACE) && !done) {
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
}
