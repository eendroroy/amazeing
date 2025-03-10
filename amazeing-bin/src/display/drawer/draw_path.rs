use crate::context::{DrawContext, CONTEXT};
use amazeing::maze::matrix::Maze;
use amazeing::DNode;
use macroquad::prelude::draw_rectangle;

pub(crate) fn draw_path(path: Vec<DNode>, ctx: &DrawContext) {
    for node in path {
        draw_rectangle(
            ctx.margin + node.1 as f32 * (ctx.cell_width + ctx.padding),
            ctx.margin + node.0 as f32 * (ctx.cell_height + ctx.padding),
            ctx.cell_width,
            ctx.cell_height,
            CONTEXT.read().unwrap().colors.color_path,
        );
    }
}

pub(crate) fn draw_current_path(path: Vec<DNode>, ctx: &DrawContext, traversed: &mut Maze) {
    for node in path {
        traversed[node] = 1;
        draw_rectangle(
            ctx.margin + node.1 as f32 * (ctx.cell_width + ctx.padding),
            ctx.margin + node.0 as f32 * (ctx.cell_height + ctx.padding),
            ctx.cell_width,
            ctx.cell_height,
            CONTEXT.read().unwrap().colors.color_visiting,
        );
    }
}
