use crate::generator::generate::{COLORS, GENERATOR_CONTEXT};
use crate::matrix::dumper::dump_maze_to_file;
use crate::matrix::loader::loader_maze_from_file;
use amazeing::maze::matrix::Maze;
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;
use std::fs;
use std::path::Path;

fn draw(maze: &Maze, margin: f32, padding: f32, cell_width: f32, cell_height: f32) {
    clear_background(COLORS.color_bg);

    for r in 0..maze.rows() {
        for c in 0..maze.cols() {
            let color: Color = if maze[(r, c)] > 0 {
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

async fn looper(maze: &mut Maze, margin: f32, padding: f32, cell_width: f32, cell_height: f32) {
    loop {
        if is_key_pressed(KeyCode::Q) {
            break;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let value = if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                0
            } else {
                1
            };

            let (mx, my) = mouse_position();
            let r = ((my - margin) / (cell_height + padding)).floor();
            let c = ((mx - margin) / (cell_width + padding)).floor();

            maze[(r as usize, c as usize)] = value;
        }

        draw(maze, margin, padding, cell_width, cell_height);
        next_frame().await
    }
}

#[macroquad::main("Maze Generator (Manually)")]
pub async fn main() {
    let (maze_file_path, rows, cols) = (
        &*GENERATOR_CONTEXT.read().unwrap().maze_file_path,
        GENERATOR_CONTEXT.read().unwrap().rows,
        GENERATOR_CONTEXT.read().unwrap().cols,
    );

    let mut maze = if fs::exists(Path::new(maze_file_path)).unwrap() {
        loader_maze_from_file(maze_file_path)
    } else {
        Maze::from(vec![vec![0u32; cols]; rows])
    };

    let (margin, padding) = (20., 3.);
    let (maze_width, maze_height) = (maze.cols(), maze.rows());
    let (cell_width, cell_height) = (15., 15.);
    let (screen_width, screen_height) = (
        margin + maze_width as f32 * (cell_width + padding) + margin,
        margin + maze_height as f32 * (cell_width + padding) + margin,
    );

    set_window_size(screen_width as u32, screen_height as u32 + 30);

    looper(&mut maze, margin, padding, cell_width, cell_height).await;

    dump_maze_to_file(maze_file_path, maze);
}
