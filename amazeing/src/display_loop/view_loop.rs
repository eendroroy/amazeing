use crate::context::{ColorContext, DrawContext, ViewContext};
use crate::helper::{delay_till_next_frame, draw_maze};
use macroquad::prelude::*;

pub(crate) async fn view_loop(context: &ViewContext, draw_context: &DrawContext, color_context: &ColorContext) {
    loop {
        draw_maze(draw_context, color_context, &context.maze, None, None, (vec![], None), false);

        if is_key_pressed(KeyCode::Q) {
            break;
        }

        delay_till_next_frame(draw_context.fps as f32);

        next_frame().await
    }
}
