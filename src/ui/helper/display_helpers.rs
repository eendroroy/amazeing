use crate::ui::component::scene::MazeScene;
use crate::ui::helper::{current_millis, dump_maze_to_file};
use macroquad::input::{KeyCode, is_key_down, is_key_pressed};
use macroquad::prelude::get_screen_data;

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

pub(crate) fn save_maze(scene: &MazeScene) {
    if (is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl)) && is_key_pressed(KeyCode::S) {
        if let Some(maze_file_path) = scene.context.maze_file_path.clone() {
            dump_maze_to_file(&maze_file_path, &scene.maze);
        }
    }
}
