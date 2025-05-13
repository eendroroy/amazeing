use crate::core::tiled::{BLOCK, OPEN};
use crate::ui::context::{AmazeingContext, Colors, DrawContext};
use crate::ui::helper::{current_millis, delay_till_next_frame, dump_maze_to_file};
use crate::ui::shape::MazeScene;
use macroquad::prelude::*;

pub(crate) async fn update_loop(
    shapes: &mut MazeScene,
    context: &AmazeingContext,
    draw_context: &DrawContext,
    color_context: &Colors,
) {
    let maze = &mut context.maze.clone();

    loop {
        let current_frame_start_time = current_millis();

        clear_background(color_context.color_bg);

        shapes.draw();

        if is_mouse_button_pressed(MouseButton::Left) {
            let (value, color) = if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                (BLOCK, color_context.color_block)
            } else {
                (OPEN, color_context.color_open)
            };

            if let Some(node) = shapes.clicked_on(mouse_position()) {
                maze[node] = value;
                shapes.update_color(node, color)
            }
        }

        if is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl) {
            if is_key_pressed(KeyCode::I) {
                get_screen_data().export_png(&format!("maze_{}_{}_{}.png", current_millis(), maze.rows(), maze.cols()));
            }

            if is_key_pressed(KeyCode::S) {
                if let Some(path) = context.maze_file_path.clone() {
                    dump_maze_to_file(path.as_path(), maze);
                }
            }
        }

        if is_key_pressed(KeyCode::Q) {
            break;
        }

        delay_till_next_frame(current_frame_start_time, draw_context.fps as f32);

        next_frame().await
    }
}
