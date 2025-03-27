use crate::context::{ColorContext, DrawContext};
use amazeing::matrix::{Maze, Node};
use macroquad::prelude::{draw_rectangle, Color};
use std::collections::HashMap;

pub(crate) fn draw_maze(
    draw_context: &DrawContext,
    color_context: &ColorContext,
    maze: &Maze,
    mut traversed: Option<&mut Maze>,
    path: Option<&HashMap<Node, bool>>,
    source: Option<Node>,
    destination: Option<Node>,
    traversing: bool,
) {
    for r in 0..maze.rows() {
        for c in 0..maze.cols() {
            let node = (r, c);
            let is_traversed = check_traversed(node, &mut traversed);
            let color: Color = if source.is_some() && source.unwrap() == node {
                color_context.color_source
            } else if destination.is_some() && destination.unwrap() == node {
                color_context.color_destination
            } else if path.is_some() && traversing && path.unwrap().get(&node).is_some() {
                if let Some(ref mut trav) = traversed {
                    trav[node] = 1;
                }
                color_context.color_visiting
            } else if path.is_some() && path.unwrap().get(&node).is_some() {
                color_context.color_path
            } else if is_traversed {
                color_context.color_traversed
            } else if maze[node] > 0 {
                color_context.color_open
            } else {
                color_context.color_block
            };

            draw_node(draw_context, node, color);
        }
    }
}

fn check_traversed(node: Node, traversed: &mut Option<&mut Maze>) -> bool {
    if let Some(t) = traversed {
        t[node] == 1
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
