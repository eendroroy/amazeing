use crate::context::DrawContext;
use amazeing::matrix::{Node, UnitShape};
use macroquad::input::mouse_position;

pub(crate) fn get_node_from_mouse_pos(ctx: &DrawContext) -> Node {
    let m = |p: f32, s: f32| ((p - ctx.margin) / (s + ctx.border)).floor() as usize;

    let (mx, my) = mouse_position();

    match ctx.u_shape {
        UnitShape::Square => (m(my, ctx.size), m(mx, ctx.size)),
        UnitShape::Hexagon | UnitShape::Circle => {
            let r = m(my, ctx.u_height);
            let c = m(mx - ctx.s(r), ctx.u_width);
            (r, c)
        }
    }
}
