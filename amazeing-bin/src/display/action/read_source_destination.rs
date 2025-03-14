use crate::helper::get_node_from_mouse_pos;
use amazeing::maze::matrix::Maze;
use amazeing::Node;
use macroquad::input::{is_key_down, KeyCode};

pub(crate) fn populate_source_destination(
    maze: &Maze,
    from: &mut Option<Node>,
    to: &mut Option<Node>,
) {
    let (r, c) = get_node_from_mouse_pos();
    if maze[(r, c)] > 0 {
        if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
            *to = Some((r, c));
        } else {
            *from = Some((r, c));
        }
    }
}
