use crate::context::{ColorContext, DrawContext};
use amazeing::matrix::{Maze, Node};
use macroquad::color::Color;
use macroquad::prelude::draw_rectangle;

pub(crate) fn draw_maze(
    draw_context: &DrawContext,
    color_context: &ColorContext,
    maze: &Maze,
    mut traversed: Option<&mut Maze>,
    path: Option<&Vec<Node>>,
    source: Option<Node>,
    destination: Option<Node>,
    traversing: bool,
) {
    for r in 0..maze.rows() {
        for c in 0..maze.cols() {
            let value = is_traversed((r, c), &mut traversed);
            let color: Color = if source.is_some() && source.unwrap() == (r, c) {
                color_context.color_source
            } else if destination.is_some() && destination.unwrap() == (r, c) {
                color_context.color_destination
            } else if path.is_some() && traversing && path.unwrap().contains(&(r, c)) {
                if let Some(ref mut trav) = traversed {
                    trav[(r, c)] = 1;
                }
                color_context.color_visiting
            } else if path.is_some() && path.unwrap().contains(&(r, c)) {
                color_context.color_path
            } else if value {
                color_context.color_traversed
            } else if maze[(r, c)] > 0 {
                color_context.color_open
            } else {
                color_context.color_block
            };

            draw_node(draw_context, (r, c), color);
        }
    }
}

fn is_traversed((r, c): Node, traversed: &mut Option<&mut Maze>) -> bool {
    if let Some(t) = traversed {
        t[(r, c)] == 1
    } else {
        false
    }
}

fn draw_node(ctx: &DrawContext, node: Node, color: Color) {
    draw_rectangle(
        ctx.margin + node.1 as f32 * (ctx.cell_width + ctx.padding),
        ctx.margin + node.0 as f32 * (ctx.cell_height + ctx.padding),
        ctx.cell_width,
        ctx.cell_height,
        color,
    );
}
