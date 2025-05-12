use crate::core::tiled::{BLOCK, OPEN};
use crate::ui::context::{ColorContext, DrawContext, ViewContext};
use crate::ui::helper::{current_millis, delay_till_next_frame, dump_maze_to_file};
use crate::ui::shape::MazeMesh;
use macroquad::prelude::*;

pub(crate) async fn update_loop(
    shapes: &mut MazeMesh,
    context: &ViewContext,
    draw_context: &DrawContext,
    color_context: &ColorContext,
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
