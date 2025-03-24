use crate::context::{ColorContext, DrawContext, SolveContext, COLOR_CTX};
use crate::helper::{draw_maze, populate_source_destination, solve_maze};
use amazeing::matrix::Node;
use macroquad::prelude::*;

pub(crate) async fn solve_loop(
    context: SolveContext,
    draw_context: &DrawContext,
    color_context: &ColorContext,
) {
    let maze = &context.maze;
    let mut current_path: Vec<Node> = vec![];
    let mut source: Option<Node> = None;
    let mut destination: Option<Node> = None;

    loop {
        clear_background(COLOR_CTX.read().unwrap().color_bg);
        if is_mouse_button_pressed(MouseButton::Left) {
            populate_source_destination(draw_context, &maze, &mut source, &mut destination);

            if source.is_some() && destination.is_some() {
                current_path = solve_maze(
                    &maze,
                    source.unwrap(),
                    destination.unwrap(),
                    &context.procedure,
                    Some(context.heuristic.clone()),
                    &mut None,
                );
            }
        }

        if is_key_pressed(KeyCode::Q) {
            break;
        }

        draw_maze(
            draw_context,
            color_context,
            &maze,
            None,
            Some(&current_path),
            source,
            destination,
            false,
        );
        next_frame().await
    }
}
