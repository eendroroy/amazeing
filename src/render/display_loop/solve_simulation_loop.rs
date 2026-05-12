use crate::maze::tiled::{Maze, Node, Trace};
use crate::render::component::scene::MazeScene;
use crate::render::helper::{current_millis, handle_mouse_click, solve_maze_stream, take_a_snap};
use macroquad::prelude::*;
use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver, TryRecvError};

enum SolveEvent {
    Step(Trace),
    Done(Vec<Node>),
}

pub(crate) async fn solve_simulation_loop(scene: &mut MazeScene) {
    let initial_maze = scene.maze.clone();
    let sources: &mut Vec<Node> = &mut vec![];
    let mut destination: Option<Node> = None;

    let mut solve_events: Option<Receiver<SolveEvent>> = None;

    let mut current_trace: Trace = HashMap::new();

    let mut trace_complete = false;
    let mut simulating = false;
    let mut paused = false;

    loop {
        let current_frame_start_time = current_millis();

        scene.clear_and_draw();

        if is_key_pressed(KeyCode::R) {
            scene.maze = initial_maze.clone();
            scene.update();
            sources.clear();
            destination = None;
            current_trace.clear();
            solve_events = None;
            trace_complete = false;
            simulating = false;
            paused = false;
        }

        if simulating {
            if !paused && !trace_complete {
                if let Some(receiver) = &solve_events {
                    // Process a small burst of frames per render so UI stays responsive.
                    for _ in 0..4 {
                        match receiver.try_recv() {
                            Ok(SolveEvent::Step(step)) => {
                                current_trace.iter().for_each(|(node, _)| {
                                    if sources.first().is_none_or(|source| source.ne(node))
                                        && destination.is_some_and(|d| d.ne(node))
                                    {
                                        scene.display_traversed(*node)
                                    }
                                });

                                current_trace = step;

                                current_trace.iter().for_each(|(node, rank)| {
                                    if sources.first().is_none_or(|source| source.ne(node))
                                        && destination.is_some_and(|d| d.ne(node))
                                    {
                                        scene.display_visiting_gradient(*node, rank)
                                    }
                                });
                            }
                            Ok(SolveEvent::Done(solution)) => {
                                trace_complete = true;
                                simulating = false;
                                solve_events = None;

                                if solution.is_empty() {
                                    current_trace.iter().for_each(|(node, _)| {
                                        if sources.first().is_none_or(|source| source.ne(node))
                                            && destination.is_some_and(|d| d.ne(node))
                                        {
                                            scene.display_traversed(*node)
                                        }
                                    });
                                } else {
                                    solution.iter().for_each(|node| {
                                        if sources.first().is_none_or(|source| source.ne(node))
                                            && destination.is_some_and(|d| d.ne(node))
                                        {
                                            scene.display_path(*node)
                                        }
                                    });
                                }
                                break;
                            }
                            Err(TryRecvError::Empty) => break,
                            Err(TryRecvError::Disconnected) => {
                                trace_complete = true;
                                simulating = false;
                                solve_events = None;
                                break;
                            }
                        }
                    }
                }
            }

            if is_key_pressed(KeyCode::Space) {
                paused = !paused;
            }
        }

        if !simulating && is_mouse_button_released(MouseButton::Left) {
            handle_mouse_click(scene, sources, &mut destination, mouse_position());
        }

        if !simulating
            && !sources.is_empty()
            && let Some(source) = sources.first()
            && let Some(destination) = destination
            && (is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::Space))
        {
            current_trace.clear();
            let worker_maze: Maze = scene.maze.clone();
            let worker_source = *source;
            let worker_destination = destination;
            let worker_procedure = scene.context.procedure.clone();
            let worker_heuristic = scene.context.heuristic;
            let (tx, rx) = mpsc::sync_channel::<SolveEvent>(512);

            std::thread::spawn(move || {
                let tx_step = tx.clone();
                let mut emit = move |step: Trace| {
                    let _ = tx_step.send(SolveEvent::Step(step));
                };
                let solution = solve_maze_stream(
                    &worker_maze,
                    worker_source,
                    worker_destination,
                    &worker_procedure,
                    worker_heuristic,
                    &mut emit,
                );
                let _ = tx.send(SolveEvent::Done(solution));
            });

            solve_events = Some(rx);
            simulating = true;
            paused = false;
        }

        take_a_snap(scene);

        if is_key_pressed(KeyCode::Q) {
            break;
        }

        scene.delay_till_next_frame(current_frame_start_time);

        next_frame().await
    }
}
