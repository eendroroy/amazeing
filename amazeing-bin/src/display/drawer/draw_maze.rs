use crate::context::{DrawContext, CONTEXT};
use amazeing::maze::matrix::Maze;
use macroquad::color::Color;
use macroquad::prelude::draw_rectangle;

pub(crate) fn draw_maze(maze: &Maze, ctx: &DrawContext) {
    for r in 0..maze.rows() {
        for c in 0..maze.cols() {
            let color: Color = if maze[(r, c)] > 0 {
                CONTEXT.read().unwrap().colors.color_open
            } else {
                CONTEXT.read().unwrap().colors.color_block
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
