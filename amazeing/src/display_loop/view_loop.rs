use crate::context::{ColorContext, DrawContext, ViewContext};
use crate::helper::draw_maze;
use macroquad::prelude::*;

pub(crate) async fn view_loop(context: &ViewContext, draw_context: &DrawContext, color_context: &ColorContext) {
    loop {
        draw_maze(draw_context, color_context, &context.maze, None, None, (vec![], None), false);

        if is_key_pressed(KeyCode::Q) {
            break;
        }

        next_frame().await
    }
}
