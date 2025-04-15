use crate::context::{ColorContext, DrawContext, ViewContext};
use crate::helper::{current_millis, delay_till_next_frame, draw_maze};
use macroquad::prelude::*;

pub(crate) async fn view_loop(context: &ViewContext, draw_context: &DrawContext, color_context: &ColorContext) {
    loop {
        let current_frame_start_time = current_millis();

        clear_background(color_context.color_bg);

        draw_maze(draw_context, color_context, &context.maze, None, None, (vec![], None), false);

        if is_key_pressed(KeyCode::Q) {
            break;
        }

        if (is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl)) && is_key_pressed(KeyCode::I) {
            get_screen_data().export_png(&format!(
                "maze_{}_{}_{}.png",
                current_millis(),
                context.maze.rows(),
                context.maze.cols()
            ));
        }

        delay_till_next_frame(current_frame_start_time, draw_context.fps as f32);

        next_frame().await
    }
}
