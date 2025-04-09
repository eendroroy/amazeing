use crate::context::{ColorContext, DrawContext, SolveContext};
use crate::helper::{
    current_millis, draw_maze, dump_text_to_file, maze_html_text, populate_source_destination, solve_maze,
};
use amazeing::matrix::{Maze, Node, Trace, Tracer};
use macroquad::prelude::*;
use std::collections::HashMap;
use std::path::Path;

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
    let mut last_millis = 0;
    let update_interval = 1000 / context.tempo as u128;

    let mut traversed = Maze::from(vec![vec![0u32; context.maze.cols()]; context.maze.rows()]);

    let mut trace_complete = false;
    let mut simulating = false;
    let mut paused = false;

    let mut trace_index = 0;

    loop {
        if is_mouse_button_pressed(MouseButton::Left) && !simulating {
            populate_source_destination(draw_context, &context.maze, &mut source, &mut destination);
        }

        if is_key_pressed(KeyCode::S) && !simulating && source.is_some() && destination.is_some() {
            solve_maze(
                &context.maze,
                &draw_context.shape,
                source.unwrap(),
                destination.unwrap(),
                &context.procedure.clone(),
                Some(context.heuristic),
                &mut tracer,
            );
            simulating = true;
            trace = tracer.clone().unwrap();
        }

        if is_key_pressed(KeyCode::Q) {
            break;
        }

        if (paused || trace_complete) && is_key_pressed(KeyCode::D) {
            let html = maze_html_text(
                &context.maze,
                Some(&mut traversed),
                Some(&current_path),
                (vec![source.unwrap()], destination),
                !trace_complete,
            );

            dump_text_to_file(Path::new(&format!("{}.html", context.maze_file_path)), html);
        }

        if simulating {
            if is_key_pressed(KeyCode::Space) {
                paused = !paused;
            }

            if !paused && !trace_complete && last_millis + update_interval <= current_millis() {
                current_path = trace.get(trace_index).unwrap().clone();
                last_millis = current_millis();
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
            )
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

        next_frame().await
    }
}
