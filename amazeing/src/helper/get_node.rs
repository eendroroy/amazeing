use crate::context::DrawContext;
use amazeing::matrix::neighbour::{DOWN, LEFT, LEFT_DOWN, LEFT_UP, RIGHT, RIGHT_DOWN, RIGHT_UP, UP};
use amazeing::matrix::{Node, UnitShape};
use macroquad::input::mouse_position;

pub(crate) fn get_node_from_mouse_pos(ctx: &DrawContext) -> Node {
    let m = |p: f32, s: f32| ((p - ctx.margin) / (s + ctx.border)).floor() as usize;

    let (mx, my) = mouse_position();

    match ctx.u_shape {
        UnitShape::Triangle => {
            let mut node = (m(my, ctx.u_height) * 2, m(mx, ctx.u_width));

            if [1usize, 2usize].contains(&(node.0 % 4)) {
                node.1 -= 1
            }

            *[LEFT, RIGHT, UP, DOWN, LEFT_UP, LEFT_DOWN, RIGHT_UP, RIGHT_DOWN]
                .iter()
                .filter_map(|i| i(node))
                .filter(|n| point_in_triangle(ctx, (mx, my), *n))
                .collect::<Vec<Node>>()
                .first()
                .unwrap_or(&node)
        }
        UnitShape::Square => (m(my, ctx.size), m(mx, ctx.size)),
        UnitShape::Hexagon | UnitShape::Circle => {
            let r = m(my, ctx.u_height);
            let c = m(mx - ctx.s(r), ctx.u_width);
            (r, c)
        }
    }
}

pub fn point_in_triangle(ctx: &DrawContext, (mx, my): (f32, f32), node: Node) -> bool {
    let (v1, v2, v3) = ctx.t_vertexes(node);

    // Calculate barycentric coordinates
    let d = (v2.y - v3.y) * (v1.x - v3.x) + (v3.x - v2.x) * (v1.y - v3.y);

    // Calculate barycentric weights
    let a = ((v2.y - v3.y) * (mx - v3.x) + (v3.x - v2.x) * (my - v3.y)) / d;
    let b = ((v3.y - v1.y) * (mx - v3.x) + (v1.x - v3.x) * (my - v3.y)) / d;
    let c = 1.0 - a - b;

    // Point is inside triangle if all weights are between 0 and 1
    (0.0..=1.0).contains(&a) && (0.0..=1.0).contains(&b) && (0.0..=1.0).contains(&c)
}
