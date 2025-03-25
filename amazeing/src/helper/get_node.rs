use crate::context::DrawContext;
use amazeing::matrix::Node;
use macroquad::input::mouse_position;

pub(crate) fn get_node_from_mouse_pos(ctx: &DrawContext) -> Node {
    let (mx, my) = mouse_position();
    let r = ((my - ctx.margin) / (ctx.cell_height + ctx.padding)).floor();
    let c = ((mx - ctx.margin) / (ctx.cell_width + ctx.padding)).floor();
    (r as usize, c as usize)
}
