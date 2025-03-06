use crate::{BG, BLOCK, OPEN};
use amazeing::solver::matrix::Maze;
use macroquad::color::Color;
use macroquad::input::{is_key_down, is_key_pressed, KeyCode};
use macroquad::prelude::{clear_background, draw_rectangle, is_mouse_button_pressed, mouse_position, next_frame, MouseButton};

pub(crate) fn draw(maze: &Maze, margin: f32, padding: f32, cell_width: f32, cell_height: f32) {
    clear_background(BG);

    for r in 0..maze.rows() {
        for c in 0..maze.cols() {
            let color: Color = if maze[(r, c)] > 0 { OPEN } else { BLOCK };

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
    maze: &mut Maze,
    margin: f32,
    padding: f32,
    cell_width: f32,
    cell_height: f32,
) {
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
