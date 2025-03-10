use crate::context::{DrawContext, COLORS};
use amazeing::maze::matrix::Maze;
use macroquad::prelude::draw_rectangle;

pub(crate) fn draw_traversed(maze: &Maze, ctx: &DrawContext) {
    for r in 0..maze.rows() {
        for c in 0..maze.cols() {
            if maze[(r, c)] == 1 {
                draw_rectangle(
                    ctx.margin + c as f32 * (ctx.cell_width + ctx.padding),
                    ctx.margin + r as f32 * (ctx.cell_height + ctx.padding),
                    ctx.cell_width,
                    ctx.cell_height,
                    COLORS.color_traversed,
                );
            };
        }
    }
}
