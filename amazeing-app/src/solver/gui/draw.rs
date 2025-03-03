use crate::{BG, BLOCK, FPS, OPEN, PATH, TRAVERSED, VISITING};
use amazeing::solver::matrix::Maze;
use macroquad::color::Color;
use macroquad::prelude::{clear_background, draw_rectangle, next_frame};
use std::thread::sleep;
use std::time::Duration;

pub(crate) fn draw(
    maze: &Maze,
    margin: f32,
    padding: f32,
    cell_width: f32,
    cell_height: f32,
    path: Vec<(usize, usize)>,
    path_color: Color,
    traversed: &mut Maze,
) {
    clear_background(BG);

    for r in 0..maze.rows() {
        for c in 0..maze.cols() {
            let color: Color = if path.contains(&(r, c)) {
                traversed[(r,c)] = 1;
                path_color
            } else if traversed[(r,c)] == 1 {
                TRAVERSED
            } else if maze[(r, c)] > 0 {
                OPEN
            } else {
                BLOCK
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
                PATH,
                &mut traversed,
            );
            next_frame().await
        } else {
            for i in 0..data.len() {
                if i == data.len() - 1 {
                    solved = true
                }

                sleep(Duration::from_millis(
                    1000u64 / FPS.lock().unwrap().clone() as u64,
                ));
                draw(
                    &maze,
                    margin,
                    padding,
                    cell_width,
                    cell_height,
                    data[i].clone(),
                    VISITING,
                    &mut traversed,
                );
                next_frame().await
            }
        }
    }
}
