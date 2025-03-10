use crate::context::DrawContext;
use crate::helper::get_node_from_mouse_pos;
use amazeing::DNode;
use macroquad::input::{is_key_down, KeyCode};

pub(crate) fn read_source_destination(ctx: &DrawContext) -> (Option<DNode>, Option<DNode>) {
    let (r, c) = get_node_from_mouse_pos(ctx);
    if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
        (None, Some((r, c)))
    } else {
        (Some((r, c)), None)
    }
}
