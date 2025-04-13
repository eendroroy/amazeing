use crate::context::{ColorContext, CreateContext, DrawContext};
use crate::helper::{add_source, delay_till_next_frame, draw_maze, dump_maze_to_file, generate_maze};
use amazeing::matrix::{Maze, Trace, Tracer};
use macroquad::prelude::*;
use std::collections::HashMap;

pub(crate) async fn generate_simulation_loop(
    context: &CreateContext,
    draw_context: &DrawContext,
    color_context: &ColorContext,
) {
    let mut traversed = Maze::from(vec![vec![0u32; context.cols]; context.rows]);
    let mut maze = Maze::from(vec![vec![0u32; context.cols]; context.rows]);
    let dummy_maze = maze.clone();

    let mut trace: Tracer = vec![];
    let mut tracer: Option<Tracer> = Some(vec![]);

    let mut path: Trace = HashMap::new();

    let mut trace_complete = false;
    let mut simulating = false;
    let mut paused = false;

    let mut sources = context.sources.clone();

    loop {
        if simulating {
            if !paused && !trace_complete {
                path = trace.remove(0);
                if trace.is_empty() {
                    trace_complete = true;
                    simulating = false;
                }
            }

            draw_maze(
                draw_context,
                color_context,
                &dummy_maze,
                Some(&mut traversed),
                Some(&path),
                (sources.clone(), None),
                true,
            );

            if is_key_pressed(KeyCode::Space) {
                paused = !paused;
            }
        } else if trace_complete {
            draw_maze(draw_context, color_context, &maze, None, None, (sources.clone(), None), false);
        } else {
            draw_maze(draw_context, color_context, &traversed, None, None, (sources.clone(), None), false);
        }

        if !simulating && !trace_complete {
            if is_mouse_button_released(MouseButton::Left) {
                add_source(draw_context, &mut sources);
            }

            if is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::Space) {
                generate_maze(&mut maze, &draw_context.unit_shape, sources.clone(), &context.procedure, &mut tracer);
                if let Some(maze_file_path) = context.maze_file_path.clone() {
                    dump_maze_to_file(&maze_file_path, &maze);
                }
                trace = tracer.clone().unwrap();
                simulating = true;
            }
        }

        if is_key_pressed(KeyCode::Q) {
            break;
        }

        delay_till_next_frame(draw_context.fps as f32);

        next_frame().await
    }
}
