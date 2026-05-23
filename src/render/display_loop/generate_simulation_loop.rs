use crate::cli::ArgProcedure;
use crate::maze::{Maze, Node, Trace, VOID};
use crate::render::helper::{current_micros, dump_maze_to_file, generate_maze_stream, take_a_snap};
use crate::render::scene::MazeScene;
use crate::render::{COLOR_SOURCE_RADIUS, FISHEYE_RADIUS, GRAVITY_WELL_RADIUS, LIGHT_RADIUS, SHOCKWAVE_RADIUS};
use macroquad::prelude::*;
use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver, TryRecvError};

enum GenerationEvent {
    Step(Trace),
    Done(Maze),
}

/// Returns true if `node` is any of the selected sources or the destination.
/// These should never be overwritten during trace rendering.
#[inline]
fn is_pinned(node: &Node, sources: &[Node], destination: Option<Node>) -> bool {
    sources.contains(node) || destination.is_some_and(|d| d == *node)
}

/// Update the per-walker trace overlay without disturbing other walkers.
///
/// Each emitted `Trace` represents the path from a walker's root (source) to its
/// current frontier node.  The root is the entry with the lowest rank value.
/// We key `active_paths` by that root so each walker gets its own independent slot.
///
/// When a slot is refreshed we only clear cells that (a) left the new path AND
/// (b) are not part of any other walker's current path — preventing cross-walker
/// flicker.
///
/// Returns the frontier node (highest rank) so the caller can update the light.
fn apply_step(
    step: Trace,
    active_paths: &mut HashMap<Node, Trace>,
    scene: &mut MazeScene,
    sources: &[Node],
    destination: Option<Node>,
) -> Option<Node> {
    // Identify walker root = node with the minimum rank in this trace.
    let root = step.iter().min_by_key(|(_, r)| **r).map(|(n, _)| *n)?;

    // Frontier node = node with the maximum rank (most recently visited).
    let frontier = step.iter().max_by_key(|(_, r)| **r).map(|(n, _)| *n);

    // Collect every node that belongs to a *different* walker's active path.
    let other_nodes: std::collections::HashSet<Node> = active_paths
        .iter()
        .filter(|(k, _)| **k != root)
        .flat_map(|(_, t)| t.keys().copied())
        .collect();

    // Clear cells that are leaving this walker's highlighted path.
    if let Some(old) = active_paths.get(&root) {
        for node in old.keys() {
            if !step.contains_key(node) && !other_nodes.contains(node) && !is_pinned(node, sources, destination) {
                scene.display_open(*node);
            }
        }
    }

    // Highlight the new path for this walker.
    for (node, rank) in &step {
        if !is_pinned(node, sources, destination) {
            scene.display_visiting_gradient(*node, rank);
        }
    }

    active_paths.insert(root, step);
    frontier
}

pub(crate) async fn generate_simulation_loop(scene: &mut MazeScene) {
    let initial_maze = scene.maze.clone();
    let mut generation_events: Option<Receiver<GenerationEvent>> = None;

    // One Trace slot per walker (keyed by walker root / source node).
    let mut active_paths: HashMap<Node, Trace> = HashMap::new();

    let mut trace_complete = false;
    let mut simulating = false;
    let mut paused = false;

    let sources: &mut Vec<Node> = &mut vec![];
    let mut destination: Option<Node> = None;

    // The node currently acting as the torch / light source.
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
            active_paths.clear();
            generation_events = None;
            trace_complete = false;
            simulating = false;
            paused = false;
            light_center = None;
        }

        if simulating {
            if is_key_pressed(KeyCode::Space) {
                paused = !paused;
            }

            if !paused
                && !trace_complete
                && let Some(receiver) = &generation_events
            {
                // Process a small burst of steps per render so UI stays responsive.
                for _ in 0..4 {
                    match receiver.try_recv() {
                        Ok(GenerationEvent::Step(step)) => {
                            if let Some(frontier) = apply_step(step, &mut active_paths, scene, sources, destination) {
                                light_center = Some(frontier);
                            }
                        }
                        Ok(GenerationEvent::Done(final_maze)) => {
                            // Clear all walker overlays before showing the finished maze.
                            for path in active_paths.values() {
                                for node in path.keys() {
                                    if !is_pinned(node, sources, destination) {
                                        scene.display_open(*node);
                                    }
                                }
                            }
                            active_paths.clear();
                            scene.maze = final_maze;
                            if let Some(maze_file_path) = scene.context.maze_file_path.clone() {
                                dump_maze_to_file(&maze_file_path, &scene.maze);
                            }
                            trace_complete = true;
                            simulating = false;
                            generation_events = None;
                            light_center = None;
                            // Restore full brightness now that the light is gone.
                            scene.restore_full_brightness();
                            scene.restore_original_positions();
                            break;
                        }
                        Err(TryRecvError::Empty) => break,
                        Err(TryRecvError::Disconnected) => {
                            trace_complete = true;
                            simulating = false;
                            generation_events = None;
                            light_center = None;
                            scene.restore_full_brightness();
                            scene.restore_original_positions();
                            break;
                        }
                    }
                }
            }
        }

        if !simulating && !trace_complete {
            if is_mouse_button_released(MouseButton::Left)
                && let Some(node) = scene.clicked_on(mouse_position())
            {
                if scene.maze[node] != VOID && !(is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift)) {
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

            if (!sources.is_empty() && (is_key_pressed(KeyCode::G) || is_key_pressed(KeyCode::Space)))
                && (!matches!(scene.context.procedure, ArgProcedure::AStar | ArgProcedure::BidirectionalAStart)
                    || destination.is_some())
            {
                active_paths.clear();
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
                // Initialise light at the first source so darkness is visible immediately.
                light_center = sources.first().copied();
            }
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

        // Apply the fish-eye zoom effect every frame while a light is active.
        if scene.context.effects.fisheye
            && let Some(center) = light_center {
                scene.apply_fisheye(center, FISHEYE_RADIUS);
            }

        // Apply the gravity-well inward-pull effect every frame while a light is active.
        if scene.context.effects.gravity_well
            && let Some(center) = light_center {
                scene.apply_gravity_well(center, GRAVITY_WELL_RADIUS);
            }

        // Apply the shockwave-pulse distortion effect every frame while a light is active.
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
