use crate::manual::draw::looper;
use crate::matrix::dumper::dump_mae_to_file;
use crate::{COLS, PATH, ROWS};
use amazeing::solver::matrix::Maze;
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;

#[macroquad::main("Maze Generator (Manually)")]
pub async fn main() {
    let mut maze = Maze::from(
        vec![vec![0u32; COLS.lock().unwrap().clone()]; ROWS.lock().unwrap().clone()]
    );

    let (margin, padding) = (20., 3.);
    let (maze_width, maze_height) = (maze.cols(), maze.rows());
    let (cell_width, cell_height) = (15., 15.);
    let (screen_width, screen_height) = (
        margin + maze_width as f32 * (cell_width + padding) + margin,
        margin + maze_height as f32 * (cell_width + padding) + margin,
    );

    set_window_size(screen_width as u32, screen_height as u32 + 30);

    looper(&mut maze, margin, padding, cell_width, cell_height).await;

    dump_mae_to_file(&PATH.lock().unwrap(), maze);
}
