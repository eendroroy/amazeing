use crate::context::DrawContext;
use crate::helper::get_node_from_mouse_pos;
use amazeing::matrix::{Maze, Node};
use macroquad::input::{KeyCode, is_key_down};

pub(crate) fn populate_source_destination(
    draw_context: &DrawContext,
    maze: &Maze,
    source: &mut Option<Node>,
    destination: &mut Option<Node>,
) {
    let (r, c) = get_node_from_mouse_pos(draw_context);
    if maze[(r, c)] > 0 {
        if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
            *destination = Some((r, c));
        } else {
            *source = Some((r, c));
        }
    }
}

pub(crate) fn add_source(draw_context: &DrawContext, sources: &mut Vec<Node>) {
    let (r, c) = get_node_from_mouse_pos(draw_context);

    if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
        if let Some(index) = sources.iter().position(|value| *value == (r, c)) {
            sources.swap_remove(index);
        }
    } else if !sources.contains(&(r, c)) {
        sources.push((r, c));
    }
}
