use crate::solver::gui::draw::looper;
use crate::{FROM, HEURISTIC, MAZE_DATA, TO};
use amazeing::solver::matrix::{
    a_star, chebyshev_heuristic, dijkstra_heuristic, euclidean_heuristic, manhattan_heuristic,
    octile_heuristic, Maze,
};
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;

#[macroquad::main("Maze Solver (A*)")]
pub async fn main() {
    let (maze, from, to) = (
        Maze::from(MAZE_DATA.lock().unwrap().clone()),
        FROM.lock().unwrap().clone(),
        TO.lock().unwrap().clone(),
    );

    let heu = match &*HEURISTIC.lock().unwrap().clone() {
        "manhattan" => manhattan_heuristic,
        "euclidean" => euclidean_heuristic,
        "chebyshev" => chebyshev_heuristic,
        "octile" => octile_heuristic,
        "dijkstra" => dijkstra_heuristic,
        other => panic!("Invalid heuristic function {}", other),
    };

    let mut tracer: Option<Vec<Vec<(usize, usize)>>> = Some(vec![]);
    a_star(&maze, from, to, heu, &mut tracer);

    let (margin, padding) = (20., 3.);
    let (maze_width, maze_height) = (maze.cols(), maze.rows());
    let (cell_width, cell_height) = (20., 20.);
    let (screen_width, screen_height) = (
        margin + maze_width as f32 * (cell_width + padding) + margin,
        margin + maze_height as f32 * (cell_width + padding) + margin,
    );

    set_window_size(screen_width as u32, screen_height as u32 + 30);

    looper(
        maze,
        margin,
        padding,
        cell_width,
        cell_height,
        tracer.clone().unwrap(),
    )
    .await
}
