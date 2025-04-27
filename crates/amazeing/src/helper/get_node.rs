use crate::context::DrawContext;
use amazeing::tiled::{Node, UnitShape};
use macroquad::input::mouse_position;

pub(crate) fn get_node_from_mouse_pos(ctx: &DrawContext, node: Node) -> Node {
    let m = |p: f32, s: f32| ((p - ctx.margin) / (s + ctx.border)).floor() as usize;

    let (mx, my) = mouse_position();

    match ctx.u_shape {
        UnitShape::Triangle => *<&Node>::clone(
            node.at(m(my, ctx.u_height) * 2, m(mx, ctx.u_width))
                .neighbours(&ctx.u_shape)
                .iter()
                .filter(|&n| point_in_triangle(ctx, (mx, my), n))
                .collect::<Vec<&Node>>()
                .first()
                .unwrap(),
        ),
        UnitShape::Square => node.at(m(my, ctx.size), m(mx, ctx.size)),
        UnitShape::Hexagon | UnitShape::Circle => {
            let r = m(my, ctx.u_height);
            let c = m(mx - ctx.s(r), ctx.u_width);
            node.at(r, c)
        }
    }
}

pub fn point_in_triangle(ctx: &DrawContext, (mx, my): (f32, f32), node: &Node) -> bool {
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
