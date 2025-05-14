use crate::command::ArgGenProcedure;
use crate::core::tiled::{Node, Trace, Tracer, VOID};
use crate::ui::component::scene::MazeScene;
use crate::ui::context::{AmazeingContext, Colors};
use crate::ui::helper::{current_millis, dump_maze_to_file, generate_maze};
use macroquad::prelude::*;
use std::collections::HashMap;

pub(crate) async fn generate_simulation_loop(scene: &mut MazeScene, context: &AmazeingContext, colors: &Colors) {
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

        scene.draw();

        if context.show_perimeter {
            scene.draw_bound();
        }

        if simulating {
            if !paused && !trace_complete {
                current_path.iter().for_each(|node| {
                    if sources.first().unwrap().ne(node.0) && (destination.is_none() || destination.unwrap().ne(node.0))
                    {
                        scene.update_color(*node.0, colors.color_open)
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
                            scene.update_color(*node.0, colors.color_open)
                        }
                    });
                } else {
                    current_path.iter().for_each(|node| {
                        if sources.first().unwrap().ne(node.0)
                            && (destination.is_none() || destination.unwrap().ne(node.0))
                        {
                            scene.update_color(*node.0, *colors.shed_color(node.1))
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
                if let Some(node) = scene.clicked_on(mouse_position()) {
                    if scene.maze[node] != VOID
                        && !(is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift))
                    {
                        if sources.contains(&node) {
                            if let Some(index) = sources.iter().position(|value| *value == node) {
                                let node = sources.swap_remove(index);
                                scene.update_color(node, colors.color_block)
                            }
                        } else {
                            sources.push(node);
                            scene.update_color(node, colors.color_source)
                        }
                    } else if scene.maze[node] != VOID
                        && (is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift))
                    {
                        if let Some(node) = destination {
                            scene.update_color(node, colors.color_block)
                        }
                        destination = Some(node);
                        scene.update_color(node, colors.color_destination)
                    }
                }
            }

            if (!sources.is_empty() && (is_key_pressed(KeyCode::G) || is_key_pressed(KeyCode::Space)))
                && (context.generation_procedure != ArgGenProcedure::AStar || destination.is_some())
            {
                generate_maze(&mut scene.maze, sources, destination, context, &mut tracer);
                if let Some(maze_file_path) = context.maze_file_path.clone() {
                    dump_maze_to_file(&maze_file_path, &scene.maze);
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

        scene.delay_till_next_frame(current_frame_start_time);

        next_frame().await
    }
}
