use crate::core::tiled::{Node, OPEN};
use crate::ui::component::scene::MazeScene;
use macroquad::prelude::{KeyCode, is_key_down};

pub fn handle_mouse_click(
    scene: &mut MazeScene,
    sources: &mut Vec<Node>,
    destination: &mut Option<Node>,
    mouse_pos: (f32, f32),
) {
    if let Some(node) = scene.clicked_on(mouse_pos) {
        if scene.maze[node] == OPEN {
            if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                if let Some(dest_node) = *destination {
                    scene.display_open(dest_node)
                }
                *destination = Some(node);
                scene.display_destination(node)
            } else {
                if let Some(src_node) = sources.first() {
                    scene.display_open(*src_node)
                }
                *sources = vec![node];
                scene.display_source(node)
            }
        }
    }
}
