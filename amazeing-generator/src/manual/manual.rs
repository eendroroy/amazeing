use crate::manual::draw::looper;
use crate::matrix::dumper::dump_maze_to_file;
use crate::matrix::loader::load_maze_data_from_file;
use crate::{COLS, PATH, ROWS};
use amazeing::solver::matrix::Maze;
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;
use std::fs;
use std::path::Path;

#[macroquad::main("Maze Generator (Manually)")]
pub async fn main() {
    let maze_data = if fs::exists(Path::new(&PATH.lock().unwrap().clone())).unwrap() {
        load_maze_data_from_file(&PATH.lock().unwrap().clone())
    } else {
        vec![vec![0u32; COLS.lock().unwrap().clone()]; ROWS.lock().unwrap().clone()]
    };

    let mut maze = Maze::from(maze_data);

    let (margin, padding) = (20., 3.);
    let (maze_width, maze_height) = (maze.cols(), maze.rows());
    let (cell_width, cell_height) = (15., 15.);
    let (screen_width, screen_height) = (
        margin + maze_width as f32 * (cell_width + padding) + margin,
        margin + maze_height as f32 * (cell_width + padding) + margin,
    );

    set_window_size(screen_width as u32, screen_height as u32 + 30);

    looper(&mut maze, margin, padding, cell_width, cell_height).await;

    dump_maze_to_file(&PATH.lock().unwrap(), maze);
}
