use crate::maze::{Maze, Node, Trace};
use crate::render::helper::{current_micros, handle_mouse_click, solve_maze_stream, take_a_snap};
use crate::render::scene::MazeScene;
use crate::render::{COLOR_SOURCE_RADIUS, FISHEYE_RADIUS, GRAVITY_WELL_RADIUS, LIGHT_RADIUS, SHOCKWAVE_RADIUS};
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

    let mut current_trace_from_source: Trace = HashMap::new();
    let mut current_trace_from_destination: Trace = HashMap::new();

    let mut trace_complete = false;
    let mut simulating = false;
    let mut paused = false;

    // The node currently acting as the torch / light source.
    // Updated to the frontier peak every time a step arrives.
    let mut light_center: Option<Node> = None;

    loop {
        let current_frame_start_time = current_micros();

        scene.clear_and_draw();

        if is_key_pressed(KeyCode::R) {
            scene.maze = initial_maze.clone();
            scene.update();
            scene.restore_original_positions();
            sources.clear();
            destination = None;
            current_trace_from_source.clear();
            current_trace_from_destination.clear();
            solve_events = None;
            trace_complete = false;
            simulating = false;
            paused = false;
            light_center = None;
        }

        if simulating {
            if !paused
                && !trace_complete
                && let Some(receiver) = &solve_events
            {
                // Process a small burst of frames per render so UI stays responsive.
                for _ in 0..4 {
                    match receiver.try_recv() {
                        Ok(SolveEvent::Step(step)) => {
                            let is_destination_side = destination.is_some_and(|d| step.contains_key(&d));

                            let active_trace = if is_destination_side {
                                &mut current_trace_from_destination
                            } else {
                                &mut current_trace_from_source
                            };

                            active_trace.iter().for_each(|(node, _)| {
                                if sources.first().is_none_or(|source| source.ne(node))
                                    && destination.is_some_and(|d| d.ne(node))
                                {
                                    scene.display_traversed(*node)
                                }
                            });

                            *active_trace = step;

                            // Update light to the frontier (highest-rank node).
                            if let Some((peak, _)) = active_trace.iter().max_by_key(|(_, r)| *r) {
                                light_center = Some(*peak);
                            }

                            active_trace.iter().for_each(|(node, rank)| {
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

                            current_trace_from_source.iter().for_each(|(node, _)| {
                                if sources.first().is_none_or(|source| source.ne(node))
                                    && destination.is_some_and(|d| d.ne(node))
                                {
                                    scene.display_traversed(*node)
                                }
                            });

                            current_trace_from_destination.iter().for_each(|(node, _)| {
                                if sources.first().is_none_or(|source| source.ne(node))
                                    && destination.is_some_and(|d| d.ne(node))
                                {
                                    scene.display_traversed(*node)
                                }
                            });

                            if solution.is_empty() {
                                // no-op: peaks already flushed to traversed above
                            } else {
                                solution.iter().for_each(|node| {
                                    if sources.first().is_none_or(|source| source.ne(node))
                                        && destination.is_some_and(|d| d.ne(node))
                                    {
                                        scene.display_path(*node)
                                    }
                                });
                            }
                            // Remove the light once solving is done.
                            light_center = None;
                            // Restore full brightness so the final result is clearly visible.
                            scene.restore_full_brightness();
                            scene.restore_original_positions();
                            break;
                        }
                        Err(TryRecvError::Empty) => break,
                        Err(TryRecvError::Disconnected) => {
                            trace_complete = true;
                            simulating = false;
                            solve_events = None;
                            light_center = None;
                            scene.restore_full_brightness();
                            scene.restore_original_positions();
                            break;
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
            current_trace_from_source.clear();
            current_trace_from_destination.clear();
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
            // Initialise light at the source so darkness is visible from frame 1.
            light_center = Some(*source);
        }

        // Apply colour effects in a single combined pass so torch, glow, and
        // chromatic-wave compose correctly (tint → dim → wave).
        let do_light = scene.context.effects.torch;
        let do_color_source = scene.context.effects.glow;
        let do_chromatic_wave = scene.context.effects.chromatic_wave;
        if (do_light || do_color_source || do_chromatic_wave)
            && let Some(center) = light_center {
                scene.apply_color_effects(
                    center,
                    LIGHT_RADIUS,
                    COLOR_SOURCE_RADIUS,
                    do_light,
                    do_color_source,
                    do_chromatic_wave,
                    get_time() as f32,
                );
            }

        // Apply the fish-eye zoom effect while simulating.
        if scene.context.effects.fisheye
            && let Some(center) = light_center {
                scene.apply_fisheye(center, FISHEYE_RADIUS);
            }

        // Apply the gravity-well inward-pull effect while simulating.
        if scene.context.effects.gravity_well
            && let Some(center) = light_center {
                scene.apply_gravity_well(center, GRAVITY_WELL_RADIUS);
            }

        // Apply the shockwave-pulse distortion effect while simulating.
        if scene.context.effects.shockwave_pulse
            && let Some(center) = light_center {
                scene.apply_shockwave(center, SHOCKWAVE_RADIUS, get_time() as f32);
            }

        take_a_snap(scene);

        if is_key_pressed(KeyCode::Q) {
            break;
        }

        scene.delay_till_next_frame(current_frame_start_time);

        next_frame().await
    }
}
