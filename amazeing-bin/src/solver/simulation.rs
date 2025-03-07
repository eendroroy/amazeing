use crate::solver::draw::looper;
use crate::solver::solve::{get_conf, SOLVER_CONTEXT};
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;

#[macroquad::main(get_conf())]
pub async fn main() {
    let maze = SOLVER_CONTEXT.read().unwrap().maze.clone();

    let (margin, padding) = (20., 3.);
    let (maze_width, maze_height) = (maze.cols(), maze.rows());
    let (cell_width, cell_height) = (15., 15.);
    let (screen_width, screen_height) = (
        margin + maze_width as f32 * (cell_width + padding) + margin,
        margin + maze_height as f32 * (cell_height + padding) + margin,
    );

    set_window_size(screen_width as u32, screen_height as u32 + 30);

    let mut trace = SOLVER_CONTEXT.read().unwrap().tracer.clone();

    looper(maze, margin, padding, cell_width, cell_height, &mut trace).await
}
