use crate::context::DrawContext;
use amazeing::matrix::{Node, Shape};
use macroquad::input::mouse_position;

pub(crate) fn get_node_from_mouse_pos(ctx: &DrawContext) -> Node {
    let m = |p: f32, s: f32| ((p - ctx.margin) / (s + ctx.border)).floor() as usize;

    let (mx, my) = mouse_position();

    match ctx.shape {
        Shape::Square => (m(my, ctx.size), m(mx, ctx.size)),
        Shape::Hexagon => {
            let r = m(my, ctx.height);
            let c = m(mx - ctx.s(r), ctx.width);
            (r, c)
        }
    }
}
