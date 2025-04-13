use crate::context::{ColorContext, DrawContext, SolveContext};
use crate::helper::{
    current_millis, delay_till_next_frame, draw_maze, path_to_trace, populate_source_destination, solve_maze,
};
use amazeing::matrix::{Node, Trace};
use macroquad::prelude::*;
use std::collections::HashMap;

pub(crate) async fn solve_loop(context: &SolveContext, draw_context: &DrawContext, color_context: &ColorContext) {
    let maze = &context.maze;
    let mut current_path: Trace = HashMap::new();
    let mut source: Option<Node> = None;
    let mut destination: Option<Node> = None;

    loop {
        draw_maze(
            draw_context,
            color_context,
            maze,
            None,
            Some(&current_path),
            (if source.is_some() { vec![source.unwrap()] } else { vec![] }, destination),
            false,
        );

        if is_mouse_button_pressed(MouseButton::Left) {
            populate_source_destination(draw_context, maze, &mut source, &mut destination);

            if source.is_some() && destination.is_some() {
                current_path = path_to_trace(solve_maze(
                    maze,
                    &draw_context.unit_shape,
                    source.unwrap(),
                    destination.unwrap(),
                    &context.procedure,
                    Some(context.heuristic),
                    &mut None,
                ));
            }
        }

        if is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl) {
            if is_key_pressed(KeyCode::I) {
                get_screen_data().export_png(&format!("maze_{}_{}_{}.png", current_millis(), maze.rows(), maze.cols()));
            }
        }

        if is_key_pressed(KeyCode::Q) {
            break;
        }

        delay_till_next_frame(draw_context.fps as f32);

        next_frame().await
    }
}
