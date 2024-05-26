mod sudoku_solver;
use macroquad::{experimental::coroutines::wait_seconds, prelude::*, window};
use sudoku_solver::*;
#[macroquad::main("Sudoku Solver")]
async fn main() {
    let grid = [
        [2, 0, 0, 5, 0, 3, 0, 0, 0],
        [8, 0, 7, 0, 2, 0, 0, 0, 0],
        [0, 0, 0, 0, 7, 0, 0, 0, 6],
        [0, 0, 5, 2, 0, 7, 6, 1, 0],
        [0, 2, 1, 6, 0, 5, 3, 0, 7],
        [3, 7, 0, 0, 0, 8, 0, 0, 0],
        [0, 0, 8, 0, 0, 0, 0, 4, 0],
        [6, 0, 2, 4, 8, 0, 0, 0, 0],
        [0, 9, 0, 7, 5, 2, 8, 6, 0],
    ];
    let mut counter: f32 = 1.0;
    let speed = 1.;
    let mut sudoku_solver = SudokuSolver::new(&grid);
    // while sudoku_solver.current_state == GAME_STATE::InProgress {  without visualization
    //     sudoku_solver.iterate();
    // }
    loop {
        clear_background(WHITE);
        let dimension_x = screen_width() / 9.0;
        let dimension_y = screen_height() / 9.0;
        if counter % speed == 0. {
            sudoku_solver.iterate();
        }
        for i in 0..9 {
            for j in 0..9 {
                let is_active = is_active_coord(&sudoku_solver, j, i);
                let is_completed_coord = sudoku_solver.is_completed_coord(j, i);
                let cell_done = sudoku_solver.grid[j as usize][i as usize] != 0;
                if is_active || is_completed_coord || cell_done {
                    let c = if is_active {
                        YELLOW
                    }else if is_completed_coord {
                        PINK
                    }else {
                        BLUE
                    };
                    draw_rectangle(
                        dimension_x * i as f32,
                        dimension_y * j as f32,
                        dimension_x,
                        dimension_y,
                        c,
                    )
                }
                draw_rectangle_lines(
                    dimension_x * i as f32,
                    dimension_y * j as f32,
                    dimension_x,
                    dimension_y,
                    1.0,
                    BLACK,
                );
                let (x, y) = get_dim(is_active);
                draw_text(
                    &i32::to_string(&sudoku_solver.grid[j as usize][i as usize]),
                    dimension_x / x + (dimension_x * i as f32),
                    dimension_y / y + (dimension_y * j as f32),
                    get_font_size(is_active),
                    get_coord_color(is_active),
                );
            }
        }
        counter += 1.;
        next_frame().await;
    }
}

fn get_coord_color(is_active: bool) -> Color {
    if is_active {
        return RED;
    }
    BLACK
}
fn get_font_size(is_active: bool) -> f32 {
    if is_active {
        return 50.;
    }
    30.
}
fn get_dim(is_active: bool) -> (f32, f32) {
    if is_active {
        return (2.5, 1.5);
    }
    (2.25, 1.75)
}
fn is_active_coord(solver: &SudokuSolver, i: i32, j: i32) -> bool {
    solver.current_grid_index.0 == i && solver.current_grid_index.1 == j
}
