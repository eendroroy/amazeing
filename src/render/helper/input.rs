use crate::maze::{Node, OPEN};
use crate::render::helper::{current_millis, dump_maze_to_file};
use crate::render::scene::MazeScene;
use macroquad::input::{KeyCode, is_key_down, is_key_pressed};
use macroquad::prelude::get_screen_data;

// ── mouse / keyboard ─────────────────────────────────────────────────────────

/// Updates `sources`/`destination` on a mouse click and highlights the node in the scene.
pub(crate) fn handle_mouse_click(
    scene: &mut MazeScene,
    sources: &mut Vec<Node>,
    destination: &mut Option<Node>,
    mouse_pos: (f32, f32),
) {
    if let Some(node) = scene.clicked_on(mouse_pos)
        && scene.maze[node] == OPEN {
            if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                if let Some(dest_node) = *destination {
                    scene.display_open(dest_node);
                }
                *destination = Some(node);
                scene.display_destination(node);
            } else {
                if let Some(src_node) = sources.first() {
                    scene.display_open(*src_node);
                }
                *sources = vec![node];
                scene.display_source(node);
            }
        }
}

// ── screenshot / save ────────────────────────────────────────────────────────

/// Exports a PNG snapshot when Ctrl+I is pressed.
pub(crate) fn take_a_snap(scene: &MazeScene) {
    if (is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl)) && is_key_pressed(KeyCode::I) {
        get_screen_data().export_png(&format!(
            "maze_{}_{}_{}.png",
            current_millis(),
            scene.maze.rows(),
            scene.maze.cols()
        ));
    }
}

/// Persists the current maze to disk when Ctrl+S is pressed.
pub(crate) fn save_maze(scene: &MazeScene) {
    if (is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl)) && is_key_pressed(KeyCode::S)
        && let Some(maze_file_path) = scene.context.maze_file_path.clone() {
            dump_maze_to_file(&maze_file_path, &scene.maze);
        }
}
