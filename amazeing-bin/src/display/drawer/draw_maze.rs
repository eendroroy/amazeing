use crate::context::{COLOR_CTX, DRAW_CTX};
use amazeing::maze::matrix::Maze;
use macroquad::color::Color;
use macroquad::prelude::draw_rectangle;

pub(crate) fn draw_maze(maze: &Maze) {
    let ctx = DRAW_CTX.read().unwrap();
    for r in 0..maze.rows() {
        for c in 0..maze.cols() {
            let color: Color = if maze[(r, c)] > 0 {
                COLOR_CTX.read().unwrap().color_open
            } else {
                COLOR_CTX.read().unwrap().color_block
            };

            draw_rectangle(
                ctx.margin + c as f32 * (ctx.cell_width + ctx.padding),
                ctx.margin + r as f32 * (ctx.cell_height + ctx.padding),
                ctx.cell_width,
                ctx.cell_height,
                color,
            );
        }
    }
}
