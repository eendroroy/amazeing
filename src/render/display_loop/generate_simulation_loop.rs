use crate::cli::ArgProcedure;
use crate::maze::tiled::{Maze, Node, Trace, VOID};
use crate::render::component::scene::MazeScene;
use crate::render::helper::{current_millis, dump_maze_to_file, generate_maze_stream, take_a_snap};
use macroquad::prelude::*;
use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver, TryRecvError};

enum GenerationEvent {
    Step(Trace),
    Done(Maze),
}

pub(crate) async fn generate_simulation_loop(scene: &mut MazeScene) {
    let initial_maze = scene.maze.clone();
    let mut generation_events: Option<Receiver<GenerationEvent>> = None;

    let mut current_path: Trace = HashMap::new();

    let mut trace_complete = false;
    let mut simulating = false;
    let mut paused = false;

    let sources: &mut Vec<Node> = &mut vec![];
    let mut destination: Option<Node> = None;

    loop {
        let current_frame_start_time = current_millis();

        scene.clear_and_draw();

        if is_key_pressed(KeyCode::R) {
            scene.maze = initial_maze.clone();
            scene.update();
            sources.clear();
            destination = None;
            current_path.clear();
            generation_events = None;
            trace_complete = false;
            simulating = false;
            paused = false;
        }

        if simulating {
            if is_key_pressed(KeyCode::Space) {
                paused = !paused;
            }

            if !paused && !trace_complete && let Some(receiver) = &generation_events {
                // Process a small burst of frames per render so UI stays responsive.
                for _ in 0..4 {
                    match receiver.try_recv() {
                        Ok(GenerationEvent::Step(step)) => {
                            current_path.iter().for_each(|(node, _)| {
                                if sources.first().is_none_or(|source| source.ne(node))
                                    && (destination.is_none() || destination.is_some_and(|d| d.ne(node)))
                                {
                                    scene.display_open(*node)
                                }
                            });

                            current_path = step;

                            current_path.iter().for_each(|(node, rank)| {
                                if sources.first().is_none_or(|source| source.ne(node))
                                    && (destination.is_none() || destination.is_some_and(|d| d.ne(node)))
                                {
                                    scene.display_visiting_gradient(*node, rank)
                                }
                            });
                        }
                        Ok(GenerationEvent::Done(final_maze)) => {
                            current_path.iter().for_each(|(node, _)| {
                                if sources.first().is_none_or(|source| source.ne(node))
                                    && (destination.is_none() || destination.is_some_and(|d| d.ne(node)))
                                {
                                    scene.display_open(*node)
                                }
                            });
                            scene.maze = final_maze;
                            if let Some(maze_file_path) = scene.context.maze_file_path.clone() {
                                dump_maze_to_file(&maze_file_path, &scene.maze);
                            }
                            trace_complete = true;
                            simulating = false;
                            generation_events = None;
                            break;
                        }
                        Err(TryRecvError::Empty) => break,
                        Err(TryRecvError::Disconnected) => {
                            trace_complete = true;
                            simulating = false;
                            generation_events = None;
                            break;
                        }
                    }
                }
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
                current_path.clear();
                let mut worker_maze = scene.maze.clone();
                let worker_sources = sources.clone();
                let worker_destination = destination;
                let worker_context = scene.context.clone();
                let (tx, rx) = mpsc::sync_channel::<GenerationEvent>(512);

                std::thread::spawn(move || {
                    let tx_step = tx.clone();
                    let mut emit = move |step: Trace| {
                        let _ = tx_step.send(GenerationEvent::Step(step));
                    };
                    generate_maze_stream(
                        &mut worker_maze,
                        &worker_sources,
                        worker_destination,
                        &worker_context,
                        &mut emit,
                    );
                    let _ = tx.send(GenerationEvent::Done(worker_maze));
                });

                generation_events = Some(rx);
                simulating = true;
                paused = false;
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
