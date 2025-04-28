use crate::context::DrawContext;
use crate::helper::get_node_from_mouse_pos;
use amazeing::tiled::{Maze, Node, OPEN, VOID};
use macroquad::input::{KeyCode, is_key_down};

pub(crate) fn populate_source_destination(
    draw_context: &DrawContext,
    maze: &Maze,
    sources: &mut Vec<Node>,
    destination: &mut Option<Node>,
) {
    if let Some(node) = get_node_from_mouse_pos(draw_context, Node::new(maze.rows(), maze.cols())) {
        if maze[node] == OPEN {
            if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                *destination = Some(node);
            } else {
                *sources = vec![node];
            }
        }
    }
}

pub(crate) fn add_source(draw_context: &DrawContext, maze: &Maze, sources: &mut Vec<Node>) {
    if let Some(node) = get_node_from_mouse_pos(draw_context, Node::new(maze.rows(), maze.cols())) {
        println!("{:?}", node);
        if maze[node] != VOID {
            if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                if let Some(index) = sources.iter().position(|value| *value == node) {
                    sources.swap_remove(index);
                }
            } else if !sources.contains(&node) {
                sources.push(node);
            }
        }
    }
}
