use crate::context::{ColorContext, DrawContext};
use amazeing::tiled::{BLOCK, Maze, Node, OPEN, Rank, Trace, UnitShape};
use macroquad::prelude::{BLANK, Color, draw_rectangle};
use macroquad::shapes::{draw_circle, draw_hexagon, draw_triangle};

pub(crate) fn draw_maze(
    draw_ctx: &DrawContext,
    colors: &ColorContext,
    maze: &Maze,
    mut traversed: Option<&mut Maze>,
    path: Option<&Trace>,
    (sources, destination): (&Vec<Node>, Option<Node>),
    traversing: bool,
) {
    let mut node = Node::new(maze.rows(), maze.cols());
    for r in 0..maze.rows() {
        for c in 0..maze.cols() {
            node = node.at(r, c);
            let rank = if let Some(path) = path { path.get(&node) } else { None };
            let is_traversed = check_traversed(node, &mut traversed);
            let color: Color = if sources.contains(&node) {
                colors.color_source
            } else if destination.is_some() && destination.unwrap() == node {
                colors.color_destination
            } else if path.is_some() && traversing && rank.is_some() {
                if let Some(ref mut trav) = traversed {
                    trav[node] = OPEN;
                }
                let idx = Rank::MAX - rank.unwrap();
                if idx < colors.color_visiting_gradient.len() as i32 {
                    *colors.color_visiting_gradient.get(idx as usize).unwrap()
                } else {
                    colors.color_visiting
                }
            } else if path.is_some() && rank.is_some() {
                colors.color_path
            } else if is_traversed {
                colors.color_traversed
            } else if maze[node] == OPEN {
                colors.color_open
            } else if maze[node] == BLOCK {
                colors.color_block
            } else {
                colors.color_bg
            };

            draw_node(draw_ctx, node, color);
        }
    }
}

fn check_traversed(node: Node, traversed: &mut Option<&mut Maze>) -> bool {
    if let Some(t) = traversed { t[node] == OPEN } else { false }
}

fn draw_node(ctx: &DrawContext, node: Node, color: Color) {
    match ctx.unit_shape {
        UnitShape::Triangle => {
            let vertexes = ctx.t_vertexes(&node);
            draw_triangle(vertexes.0, vertexes.1, vertexes.2, color);
        }
        UnitShape::Square => draw_rectangle(ctx.x(node), ctx.y(node), ctx.size, ctx.size, color),
        UnitShape::Hexagon => draw_hexagon(ctx.x(node), ctx.y(node), ctx.size, 0., true, BLANK, color),
        UnitShape::Circle => draw_circle(ctx.x(node), ctx.y(node), ctx.size, color),
    }
}
