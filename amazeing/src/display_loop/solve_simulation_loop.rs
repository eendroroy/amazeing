use crate::context::{ColorContext, DrawContext, SolveContext};
use crate::helper::{current_millis, delay_till_next_frame, draw_maze, populate_source_destination, solve_maze};
use amazeing::matrix::{Maze, Node, Trace, Tracer};
use macroquad::prelude::*;
use std::collections::HashMap;

pub(crate) async fn solve_simulation_loop(
    context: &SolveContext,
    draw_context: &DrawContext,
    color_context: &ColorContext,
) {
    let mut source: Option<Node> = None;
    let mut destination: Option<Node> = None;

    let mut trace: Tracer = vec![];
    let mut tracer: Option<Tracer> = Some(vec![]);

    let mut current_path: Trace = HashMap::new();

    let mut traversed = Maze::from(vec![vec![0u32; context.maze.cols()]; context.maze.rows()]);

    let mut trace_complete = false;
    let mut simulating = false;
    let mut paused = false;

    let mut trace_index = 0;

    loop {
        let current_frame_start_time = current_millis();

        clear_background(color_context.color_bg);

        if simulating {
            if !paused && !trace_complete {
                current_path = trace.get(trace_index).unwrap().clone();
                trace_index += 1;
                if trace.len() == trace_index {
                    trace_complete = true;
                }
            }

            draw_maze(
                draw_context,
                color_context,
                &context.maze,
                Some(&mut traversed),
                Some(&current_path),
                (vec![source.unwrap()], destination),
                !trace_complete,
            );

            if is_key_pressed(KeyCode::Space) {
                paused = !paused;
            }
        } else {
            draw_maze(
                draw_context,
                color_context,
                &context.maze,
                None,
                None,
                (if source.is_some() { vec![source.unwrap()] } else { vec![] }, destination),
                false,
            );
        }

        if !simulating && is_mouse_button_released(MouseButton::Left) {
            populate_source_destination(draw_context, &context.maze, &mut source, &mut destination);
        }

        if !simulating
            && source.is_some()
            && destination.is_some()
            && (is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::Space))
        {
            solve_maze(
                &context.maze,
                &draw_context.u_shape,
                source.unwrap(),
                destination.unwrap(),
                &context.procedure.clone(),
                Some(context.heuristic),
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
