use crate::solver::solve::{COLORS, SOLVER_CONTEXT};
use amazeing::solver::matrix::Maze;
use macroquad::color::Color;
use macroquad::input::{is_key_pressed, KeyCode};
use macroquad::prelude::{clear_background, draw_rectangle, next_frame};
use std::time::{SystemTime, UNIX_EPOCH};

fn current_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

pub(crate) fn draw_simulation(
    maze: &Maze,
    margin: f32,
    padding: f32,
    cell_width: f32,
    cell_height: f32,
    path: Vec<(usize, usize)>,
    path_color: Color,
    traversed: &mut Maze,
) {
    clear_background(COLORS.color_bg);

    for r in 0..maze.rows() {
        for c in 0..maze.cols() {
            let color: Color = if SOLVER_CONTEXT.read().unwrap().source == (r, c) {
                COLORS.color_source
            } else if SOLVER_CONTEXT.read().unwrap().destination == (r, c) {
                COLORS.color_destination
            } else if path.contains(&(r, c)) {
                traversed[(r, c)] = 1;
                path_color
            } else if traversed[(r, c)] == 1 {
                COLORS.color_traversed
            } else if maze[(r, c)] > 0 {
                COLORS.color_open
            } else {
                COLORS.color_block
            };

            draw_rectangle(
                margin + c as f32 * (cell_width + padding),
                margin + r as f32 * (cell_height + padding),
                cell_width,
                cell_height,
                color,
            );
        }
    }
}

pub(crate) fn draw_maze(maze: &Maze, margin: f32, padding: f32, cell_width: f32, cell_height: f32) {
    clear_background(COLORS.color_bg);

    for r in 0..maze.rows() {
        for c in 0..maze.cols() {
            let color: Color = if SOLVER_CONTEXT.read().unwrap().source == (r, c) {
                COLORS.color_source
            } else if SOLVER_CONTEXT.read().unwrap().destination == (r, c) {
                COLORS.color_destination
            } else if maze[(r, c)] > 0 {
                COLORS.color_open
            } else {
                COLORS.color_block
            };

            draw_rectangle(
                margin + c as f32 * (cell_width + padding),
                margin + r as f32 * (cell_height + padding),
                cell_width,
                cell_height,
                color,
            );
        }
    }
}

pub(crate) async fn looper(
    maze: Maze,
    margin: f32,
    padding: f32,
    cell_width: f32,
    cell_height: f32,
    trace: &mut Vec<Vec<(usize, usize)>>,
) {
    let mut current_path: Vec<(usize, usize)> = vec![];
    let mut last_millis = current_millis();
    let update_interval = 1000 / SOLVER_CONTEXT.read().unwrap().fps as u128;

    let mut traversed: Maze = Maze::from(vec![vec![0u32; maze.cols()]; maze.rows()]);

    let mut trace_complete = false;
    let mut simulating = false;

    loop {
        if is_key_pressed(KeyCode::S) && !simulating {
            println!("Starting Simulation");
            simulating = true;
            current_path = trace.remove(0);
            last_millis = current_millis();
            if trace.len() == 0 {
                trace_complete = true;
            }
        }

        if is_key_pressed(KeyCode::Q) {
            println!("Quitting");
            break;
        }

        if simulating {
            if !trace_complete && last_millis + update_interval <= current_millis() {
                current_path = trace.remove(0);
                last_millis = current_millis();
                if trace.len() == 0 {
                    trace_complete = true;
                }
            }

            let color = if trace_complete {
                COLORS.color_path
            } else {
                COLORS.color_visiting
            };

            draw_simulation(
                &maze,
                margin,
                padding,
                cell_width,
                cell_height,
                current_path.clone(),
                color,
                &mut traversed,
            );
        } else {
            draw_maze(&maze, margin, padding, cell_width, cell_height);
        }
        next_frame().await
    }
}
