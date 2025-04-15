use crate::context::{ColorContext, DrawContext, ViewContext};
use crate::helper::{current_millis, delay_till_next_frame, draw_maze, dump_maze_to_file, get_node_from_mouse_pos};
use macroquad::prelude::*;

pub(crate) async fn update_loop(context: &ViewContext, draw_context: &DrawContext, color_context: &ColorContext) {
    let maze = &mut context.maze.clone();

    loop {
        let current_frame_start_time = current_millis();

        clear_background(color_context.color_bg);

        draw_maze(draw_context, color_context, maze, None, None, (vec![], None), false);

        if is_mouse_button_pressed(MouseButton::Left) {
            let value = if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                0
            } else {
                1
            };

            let node = get_node_from_mouse_pos(draw_context);

            maze[node] = value;
        }

        if is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl) {
            if is_key_pressed(KeyCode::I) {
                get_screen_data().export_png(&format!("maze_{}_{}_{}.png", current_millis(), maze.rows(), maze.cols()));
            }

            if is_key_pressed(KeyCode::S) {
                dump_maze_to_file(&context.maze_file_path, maze);
            }
        }

        if is_key_pressed(KeyCode::Q) {
            break;
        }

        delay_till_next_frame(current_frame_start_time, draw_context.fps as f32);

        next_frame().await
    }
}
