use crate::core::tiled::{Node, Trace, Tracer};
use crate::ui::component::scene::MazeScene;
use crate::ui::helper::{current_millis, handle_mouse_click, solve_maze, take_a_snap};
use macroquad::prelude::*;
use std::collections::HashMap;

pub(crate) async fn solve_simulation_loop(scene: &mut MazeScene) {
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

        scene.clear_and_draw();

        if simulating {
            if !paused && !trace_complete {
                current_trace.iter().for_each(|(node, _)| {
                    if sources.first().unwrap().ne(node) && destination.unwrap().ne(node) {
                        scene.display_traversed(*node)
                    }
                });
                current_trace = trace.get(trace_index).unwrap().clone();
                trace_index += 1;
                if trace.len() == trace_index {
                    trace_complete = true;
                    current_trace.iter().for_each(|node| {
                        if sources.first().unwrap().ne(node.0) && destination.unwrap().ne(node.0) {
                            scene.display_path(*node.0)
                        }
                    });
                } else {
                    current_trace.iter().for_each(|node| {
                        if sources.first().unwrap().ne(node.0) && destination.unwrap().ne(node.0) {
                            scene.display_visiting_gradient(*node.0, node.1)
                        }
                    });
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
            solve_maze(
                &scene.maze,
                *source,
                destination,
                &scene.context.procedure.clone(),
                scene.context.heuristic,
                &mut tracer,
            );
            simulating = true;
            trace = tracer.clone().unwrap();
        }

        take_a_snap(scene);

        if is_key_pressed(KeyCode::Q) {
            break;
        }

        scene.delay_till_next_frame(current_frame_start_time);

        next_frame().await
    }
}
