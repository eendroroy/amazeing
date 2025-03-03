use crate::solver::gui::draw::draw;
use crate::solver::matrix::loader::{loader_maze_from_file, parse_node};
use crate::{FROM, PATH, TO};
use amazeing::solver::matrix::dfs;
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;
use std::thread::sleep;
use std::time::Duration;

#[macroquad::main("Maze Solver (DFS)")]
pub async fn main() {
    let (maze, from, to) = unsafe {
        (
            loader_maze_from_file(&*PATH),
            parse_node(&*FROM),
            parse_node(&*TO),
        )
    };

    let mut tracer: Option<Vec<Vec<(usize, usize)>>> = Some(vec![]);
    dfs(&maze, from, to, &mut tracer);

    let (margin, padding) = (20., 3.);
    let (maze_width, maze_height) = (maze.cols(), maze.rows());
    let (cell_width, cell_height) = (20., 20.);
    let (screen_width, screen_height) = (
        margin + maze_width as f32 * (cell_width + padding) + margin,
        margin + maze_height as f32 * (cell_width + padding) + margin,
    );

    set_window_size(screen_width as u32, screen_height as u32 + 30);

    let data = tracer.clone().unwrap();
    let mut solved = false;

    loop {
        if solved {
            draw(
                &maze,
                margin,
                padding,
                cell_width,
                cell_height,
                data.last().unwrap().clone(),
                GOLD,
            );
            next_frame().await
        } else {
            for i in 0..data.len() {
                if i == data.len() - 1 {
                    solved = true
                }

                sleep(Duration::from_millis(150));
                draw(
                    &maze,
                    margin,
                    padding,
                    cell_width,
                    cell_height,
                    data[i].clone(),
                    MAGENTA,
                );
                next_frame().await
            }
        }
    }
}
