use crate::core::tiled::{Node, Trace, Tracer, OPEN};
use crate::ui::context::{Colors, DrawContext, SolveContext};
use crate::ui::helper::{current_millis, delay_till_next_frame, solve_maze};
use crate::ui::shape::MazeScene;
use macroquad::prelude::*;
use std::collections::HashMap;

pub(crate) async fn solve_simulation_loop(
    shapes: &mut MazeScene,
    context: &SolveContext,
    draw_context: &DrawContext,
    colors: &Colors,
) {
    let sources: &mut Vec<Node> = &mut vec![];
    let mut destination: Option<Node> = None;

    let mut trace: Tracer = vec![];
    let mut tracer: Option<Tracer> = Some(vec![]);

    let mut current_trace: Trace = HashMap::new();

    let mut trace_complete = false;
    let mut simulating = false;
    let mut paused = false;

    let mut trace_index = 0;

    loop {
        let current_frame_start_time = current_millis();

        clear_background(colors.color_bg);

        shapes.draw();

        if simulating {
            if !paused && !trace_complete {
                current_trace.iter().for_each(|(node, _)| {
                    if sources.first().unwrap().ne(node) && destination.unwrap().ne(node) {
                        shapes.update_color(*node, colors.color_traversed)
                    }
                });
                current_trace = trace.get(trace_index).unwrap().clone();
                trace_index += 1;
                if trace.len() == trace_index {
                    trace_complete = true;
                    current_trace.iter().for_each(|node| {
                        if sources.first().unwrap().ne(node.0) && destination.unwrap().ne(node.0) {
                            shapes.update_color(*node.0, colors.color_path)
                        }
                    });
                } else {
                    current_trace.iter().for_each(|node| {
                        if sources.first().unwrap().ne(node.0) && destination.unwrap().ne(node.0) {
                            shapes.update_color(*node.0, *colors.shed_color(node.1))
                        }
                    });
                }
            }

            if is_key_pressed(KeyCode::Space) {
                paused = !paused;
            }
        }

        if !simulating && is_mouse_button_released(MouseButton::Left) {
            if let Some(node) = shapes.clicked_on(mouse_position()) {
                if context.maze[node] == OPEN {
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
        }

        if !simulating
            && !sources.is_empty()
            && destination.is_some()
            && (is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::Space))
        {
            solve_maze(
                &context.maze,
                &draw_context.unit_shape,
                *sources.first().unwrap(),
                destination.unwrap(),
                &context.procedure.clone(),
                context.heuristic,
                &mut tracer,
            );
            simulating = true;
            trace = tracer.clone().unwrap();
        }

        if (is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl)) && is_key_pressed(KeyCode::I) {
            get_screen_data().export_png(&format!(
                "maze_{}_{}_{}.png",
                current_millis(),
                context.maze.rows(),
                context.maze.cols()
            ));
        }

        if is_key_pressed(KeyCode::Q) {
            break;
        }

        delay_till_next_frame(current_frame_start_time, draw_context.fps as f32);

        next_frame().await
    }
}
