use crate::ui::component::scene::MazeScene;
use crate::ui::context::{AmazeingContext, Colors, DrawContext};
use crate::ui::helper::{current_millis, delay_till_next_frame};
use macroquad::prelude::*;

pub(crate) async fn view_loop(
    shapes: MazeScene,
    context: &AmazeingContext,
    draw_context: &DrawContext,
    color_context: &Colors,
) {
    loop {
        let current_frame_start_time = current_millis();

        clear_background(color_context.color_bg);

        shapes.draw();

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
