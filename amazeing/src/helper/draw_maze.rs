use crate::context::{ColorContext, DrawContext};
use amazeing::matrix::{Maze, Node, Rank, Shape, Trace};
use macroquad::prelude::{Color, draw_poly, draw_rectangle};

pub(crate) fn draw_maze(
    draw_context: &DrawContext,
    color_context: &ColorContext,
    maze: &Maze,
    mut traversed: Option<&mut Maze>,
    path: Option<&Trace>,
    (sources, destination): (Vec<Node>, Option<Node>),
    traversing: bool,
) {
    for r in 0..maze.rows() {
        for c in 0..maze.cols() {
            let node = (r, c);
            let rank = if let Some(path) = path { path.get(&node) } else { None };
            let is_traversed = check_traversed(node, &mut traversed);
            let color: Color = if sources.contains(&node) {
                color_context.color_source
            } else if destination.is_some() && destination.unwrap() == node {
                color_context.color_destination
            } else if path.is_some() && traversing && rank.is_some() {
                if let Some(ref mut trav) = traversed {
                    trav[node] = 1;
                }
                let idx = Rank::MAX - rank.unwrap();
                if idx < color_context.color_visiting_gradient.len() as i32 {
                    *color_context.color_visiting_gradient.get(idx as usize).unwrap()
                } else {
                    color_context.color_visiting
                }
            } else if path.is_some() && rank.is_some() {
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
    if let Some(t) = traversed { t[node] == 1 } else { false }
}

fn draw_node(ctx: &DrawContext, node: Node, color: Color) {
    match ctx.shape {
        Shape::Square => draw_rectangle(ctx.x(node), ctx.y(node), ctx.size, ctx.size, color),
        Shape::Hexagon => draw_poly(ctx.x(node), ctx.y(node), 6, ctx.size, 90., color),
    }
}
