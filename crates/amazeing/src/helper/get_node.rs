use crate::context::DrawContext;
use amazeing::tiled::{Node, UnitShape};
use macroquad::input::mouse_position;
use macroquad::math::Vec2;

pub(crate) fn get_node_from_mouse_pos(ctx: &DrawContext, node: Node) -> Option<Node> {
    let m = |p: f32, s: f32| ((p - ctx.margin) / (s + ctx.border)).floor() as usize;

    let (mx, my) = mouse_position();

    match ctx.unit_shape {
        UnitShape::Triangle => {
            if let Some(node) = node.at(m(my, ctx.unit_height) * 2, m(mx, ctx.unit_width)) {
                if point_in_triangle(ctx.t_vertexes(&node), (mx, my)) {
                    Some(node)
                } else {
                    let neighbours = node.surroundings();

                    let o_node = neighbours
                        .iter()
                        .filter(|&n| point_in_triangle(ctx.t_vertexes(n), (mx, my)))
                        .collect::<Vec<&Node>>()
                        .first()
                        .cloned();
                    if let Some(&node) = o_node { Some(node) } else { None }
                }
            } else {
                None
            }
        }
        UnitShape::Square => node.at(m(my, ctx.size), m(mx, ctx.size)),
        UnitShape::Hexagon => {
            let r = m(my, ctx.unit_height);
            let c = m(mx - ctx.s(r), ctx.unit_width);
            node.at(r, c)
        }
    }
}

pub fn point_in_triangle((v1, v2, v3): (Vec2, Vec2, Vec2), (mx, my): (f32, f32)) -> bool {
    // Calculate barycentric coordinates
    let d = (v2.y - v3.y) * (v1.x - v3.x) + (v3.x - v2.x) * (v1.y - v3.y);

    // Calculate barycentric weights
    let a = ((v2.y - v3.y) * (mx - v3.x) + (v3.x - v2.x) * (my - v3.y)) / d;
    let b = ((v3.y - v1.y) * (mx - v3.x) + (v1.x - v3.x) * (my - v3.y)) / d;
    let c = 1.0 - a - b;

    // Point is inside triangle if all weights are between 0 and 1
    (0.0..=1.0).contains(&a) && (0.0..=1.0).contains(&b) && (0.0..=1.0).contains(&c)
}
