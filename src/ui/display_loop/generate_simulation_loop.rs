use crate::command::ArgGenProcedure;
use crate::core::tiled::{Maze, Node, Trace, Tracer, VOID};
use crate::ui::context::{ColorContext, CreateContext, DrawContext};
use crate::ui::helper::{current_millis, delay_till_next_frame, dump_maze_to_file, generate_maze};
use crate::ui::shape::maze_mesh::MazeMesh;
use macroquad::prelude::*;
use std::collections::HashMap;

pub(crate) async fn generate_simulation_loop(
    shapes: &mut MazeMesh,
    maze: &mut Maze,
    context: &CreateContext,
    draw_context: &DrawContext,
    colors: &ColorContext,
) {
    let mut trace: Tracer = vec![];
    let mut tracer: Option<Tracer> = Some(vec![]);

    let mut current_path: Trace = HashMap::new();

    let mut trace_complete = false;
    let mut simulating = false;
    let mut paused = false;

    let sources: &mut Vec<Node> = &mut vec![];
    let mut destination: Option<Node> = None;

    loop {
        let current_frame_start_time = current_millis();

        clear_background(colors.color_bg);

        shapes.draw();

        if simulating {
            if !paused && !trace_complete {
                current_path.iter().for_each(|node| {
                    if sources.first().unwrap().ne(node.0) && (destination.is_none() || destination.unwrap().ne(node.0))
                    {
                        shapes.update_color(*node.0, colors.color_open)
                    }
                });
                current_path = trace.remove(0);
                if trace.is_empty() {
                    trace_complete = true;
                    simulating = false;
                    current_path.iter().for_each(|node| {
                        if sources.first().unwrap().ne(node.0)
                            && (destination.is_none() || destination.unwrap().ne(node.0))
                        {
                            shapes.update_color(*node.0, colors.color_open)
                        }
                    });
                } else {
                    current_path.iter().for_each(|node| {
                        if sources.first().unwrap().ne(node.0)
                            && (destination.is_none() || destination.unwrap().ne(node.0))
                        {
                            shapes.update_color(*node.0, *colors.shed_color(node.1).unwrap_or(&colors.color_visiting))
                        }
                    });
                }
            }

            if is_key_pressed(KeyCode::Space) {
                paused = !paused;
            }
        }

        if !simulating && !trace_complete {
            if is_mouse_button_released(MouseButton::Left) {
                if let Some(node) = shapes.clicked_on(mouse_position()) {
                    if maze[node] != VOID && !(is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift)) {
                        if sources.contains(&node) {
                            if let Some(index) = sources.iter().position(|value| *value == node) {
                                let node = sources.swap_remove(index);
                                shapes.update_color(node, colors.color_block)
                            }
                        } else {
                            sources.push(node);
                            shapes.update_color(node, colors.color_source)
                        }
                    } else if maze[node] != VOID
                        && (is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift))
                    {
                        if let Some(node) = destination {
                            shapes.update_color(node, colors.color_block)
                        }
                        destination = Some(node);
                        shapes.update_color(node, colors.color_destination)
                    }
                }
            }

            if (!sources.is_empty() && (is_key_pressed(KeyCode::G) || is_key_pressed(KeyCode::Space)))
                && (context.procedure != ArgGenProcedure::AStar || destination.is_some())
            {
                generate_maze(maze, &draw_context.unit_shape, sources, destination, context, &mut tracer);
                if let Some(maze_file_path) = context.maze_file_path.clone() {
                    dump_maze_to_file(&maze_file_path, maze);
                }
                trace = tracer.clone().unwrap();
                simulating = true;
            }
        }

        if (is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl)) && is_key_pressed(KeyCode::I) {
            get_screen_data().export_png(&format!("maze_{}_{}_{}.png", current_millis(), context.rows, context.cols));
        }

        if is_key_pressed(KeyCode::Q) {
            break;
        }

        delay_till_next_frame(current_frame_start_time, draw_context.fps as f32);

        next_frame().await
    }
}
