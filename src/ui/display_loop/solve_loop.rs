use crate::core::tiled::{Node, OPEN};
use crate::ui::component::scene::MazeScene;
use crate::ui::context::{AmazeingContext, Colors, DrawContext};
use crate::ui::helper::{current_millis, delay_till_next_frame, solve_maze};
use macroquad::prelude::*;

pub(crate) async fn solve_loop(
    shapes: &mut MazeScene,
    context: &AmazeingContext,
    draw_context: &DrawContext,
    colors: &Colors,
) {
    let maze = &context.maze;
    let sources = &mut vec![];
    let mut destination: Option<Node> = None;
    let mut path: Vec<Node> = vec![];

    loop {
        let current_frame_start_time = current_millis();

        clear_background(colors.color_bg);

        shapes.draw();

        if is_mouse_button_pressed(MouseButton::Left) {
            if let Some(node) = shapes.clicked_on(mouse_position()) {
                if maze[node] == OPEN {
                    if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                        if let Some(node) = destination {
                            shapes.update_color(node, colors.color_open)
                        }
                        destination = Some(node);
                        shapes.update_color(node, colors.color_destination)
                    } else {
                        if let Some(node) = sources.first() {
                            shapes.update_color(*node, colors.color_open)
                        }
                        *sources = vec![node];
                        shapes.update_color(node, colors.color_source)
                    }
                }
            }

            if !sources.is_empty() && destination.is_some() {
                path.iter().for_each(|node| {
                    if sources.first().unwrap().ne(node) && destination.unwrap().ne(node) {
                        shapes.update_color(*node, colors.color_open)
                    }
                });

                path = solve_maze(
                    maze,
                    &draw_context.unit_shape,
                    *sources.first().unwrap(),
                    destination.unwrap(),
                    &context.solve_procedure,
                    context.heuristic,
                    &mut None,
                );

                path.iter().for_each(|node| {
                    if sources.first().unwrap().ne(node) && destination.unwrap().ne(node) {
                        shapes.update_color(*node, colors.color_path)
                    }
                })
            }
        }

        if (is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl)) && is_key_pressed(KeyCode::I) {
            get_screen_data().export_png(&format!("maze_{}_{}_{}.png", current_millis(), maze.rows(), maze.cols()));
        }

        if is_key_pressed(KeyCode::Q) {
            break;
        }

        delay_till_next_frame(current_frame_start_time, draw_context.fps as f32);

        next_frame().await
    }
}
