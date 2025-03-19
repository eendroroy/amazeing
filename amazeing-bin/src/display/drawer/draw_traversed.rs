use crate::context::{COLOR_CTX, DRAW_CTX};
use amazeing::matrix::Maze;
use macroquad::prelude::draw_rectangle;

pub(crate) fn draw_traversed(maze: &Maze) {
    let ctx = DRAW_CTX.read().unwrap();
    for r in 0..maze.rows() {
        for c in 0..maze.cols() {
            if maze[(r, c)] == 1 {
                draw_rectangle(
                    ctx.margin + c as f32 * (ctx.cell_width + ctx.padding),
                    ctx.margin + r as f32 * (ctx.cell_height + ctx.padding),
                    ctx.cell_width,
                    ctx.cell_height,
                    COLOR_CTX.read().unwrap().color_traversed,
                );
            };
        }
    }
}
