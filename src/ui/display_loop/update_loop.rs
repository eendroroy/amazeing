use crate::core::tiled::{BLOCK, OPEN};
use crate::ui::component::scene::MazeScene;
use crate::ui::context::{AmazeingContext, Colors};
use crate::ui::helper::{current_millis, dump_maze_to_file};
use macroquad::prelude::*;

pub(crate) async fn update_loop(scene: &mut MazeScene, context: &AmazeingContext, color_context: &Colors) {
    loop {
        let current_frame_start_time = current_millis();

        clear_background(color_context.color_bg);

        scene.draw();

        if is_mouse_button_pressed(MouseButton::Left) {
            let (value, color) = if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                (BLOCK, color_context.color_block)
            } else {
                (OPEN, color_context.color_open)
            };

            if let Some(node) = scene.clicked_on(mouse_position()) {
                scene.maze[node] = value;
                scene.update_color(node, color)
            }
        }

        if is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl) {
            if is_key_pressed(KeyCode::I) {
                get_screen_data().export_png(&format!(
                    "maze_{}_{}_{}.png",
                    current_millis(),
                    scene.maze.rows(),
                    scene.maze.cols()
                ));
            }

            if is_key_pressed(KeyCode::S) {
                if let Some(path) = context.maze_file_path.clone() {
                    dump_maze_to_file(path.as_path(), &scene.maze);
                }
            }
        }

        if is_key_pressed(KeyCode::Q) {
            break;
        }

        scene.delay_till_next_frame(current_frame_start_time);

        next_frame().await
    }
}
