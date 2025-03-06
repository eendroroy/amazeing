use amazeing::solver::matrix::Maze;
use macroquad::color::Color;
use macroquad::input::{is_key_pressed, KeyCode};
use macroquad::prelude::{clear_background, draw_rectangle, next_frame};
use std::thread::sleep;
use std::time::Duration;
use crate::solve::{COLORS, SOLVER_CONTEXT};

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
    data: Vec<Vec<(usize, usize)>>,
) {
    let mut traversed: Maze = Maze::from(vec![vec![0u32; maze.cols()]; maze.rows()]);
    let mut end = false;
    let mut start = false;
    loop {
        if is_key_pressed(KeyCode::S) {
            start = true;
        }
        if start {
            if end {
                draw_simulation(
                    &maze,
                    margin,
                    padding,
                    cell_width,
                    cell_height,
                    data.last().unwrap().clone(),
                    COLORS.color_path,
                    &mut traversed,
                );
                next_frame().await
            } else {
                for i in 0..data.len() {
                    if i == data.len() - 1 {
                        end = true
                    }

                    sleep(Duration::from_millis(
                        1000u64 / SOLVER_CONTEXT.read().unwrap().fps as u64,
                    ));
                    draw_simulation(
                        &maze,
                        margin,
                        padding,
                        cell_width,
                        cell_height,
                        data[i].clone(),
                        COLORS.color_visiting,
                        &mut traversed,
                    );
                    next_frame().await
                }
            }
        } else {
            draw_maze(&maze, margin, padding, cell_width, cell_height);
            next_frame().await
        }
    }
}
