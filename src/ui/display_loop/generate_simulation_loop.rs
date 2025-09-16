use crate::command::ArgProcedure;
use crate::core::tiled::{Node, Trace, Tracer, VOID};
use crate::ui::component::scene::MazeScene;
use crate::ui::helper::{current_millis, dump_maze_to_file, generate_maze, take_a_snap};
use macroquad::prelude::*;
use std::collections::HashMap;

pub(crate) async fn generate_simulation_loop(scene: &mut MazeScene) {
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

        scene.clear_and_draw();

        if simulating {
            if !paused && !trace_complete {
                current_path.iter().for_each(|node| {
                    if sources.first().unwrap().ne(node.0) && (destination.is_none() || destination.unwrap().ne(node.0))
                    {
                        scene.display_open(*node.0)
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
                            scene.display_open(*node.0)
                        }
                    });
                } else {
                    current_path.iter().for_each(|node| {
                        if sources.first().unwrap().ne(node.0)
                            && (destination.is_none() || destination.unwrap().ne(node.0))
                        {
                            scene.display_visiting_gradient(*node.0, node.1)
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
                                scene.display_block(node)
                            }
                        } else {
                            sources.push(node);
                            scene.display_source(node)
                        }
                    } else if scene.maze[node] != VOID
                        && (is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift))
                    {
                        if let Some(node) = destination {
                            scene.display_block(node)
                        }
                        destination = Some(node);
                        scene.display_destination(node)
                    }
                }
            }

            if (!sources.is_empty() && (is_key_pressed(KeyCode::G) || is_key_pressed(KeyCode::Space)))
                && (scene.context.procedure != ArgProcedure::AStar || destination.is_some())
            {
                generate_maze(&mut scene.maze, sources, destination, &scene.context, &mut tracer);
                if let Some(maze_file_path) = scene.context.maze_file_path.clone() {
                    dump_maze_to_file(&maze_file_path, &scene.maze);
                }
                trace = tracer.clone().unwrap();
                simulating = true;
            }
        }

        take_a_snap(scene);

        if is_key_pressed(KeyCode::Q) {
            break;
        }

        scene.delay_till_next_frame(current_frame_start_time);

        next_frame().await
    }
}
