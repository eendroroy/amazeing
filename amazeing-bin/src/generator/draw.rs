use crate::generator::generate::COLORS;
use amazeing::maze::matrix::Maze;
use macroquad::color::Color;
use macroquad::prelude::{clear_background, draw_rectangle};

pub(crate) fn draw_maze(maze: &Maze, margin: f32, padding: f32, cell_width: f32, cell_height: f32) {
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
