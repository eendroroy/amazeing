use crate::context::{ColorContext, CreateContext, DrawContext};
use crate::helper::{current_millis, draw_maze, dump_maze_to_file, generate_maze};
use amazeing::matrix::{Maze, Node, Tracer};
use macroquad::prelude::*;

pub(crate) async fn generate_simulation_loop(
    context: CreateContext,
    draw_context: &DrawContext,
    color_context: &ColorContext,
) {
    let visited = Maze::from(vec![vec![0u32; context.cols]; context.rows]);
    let mut maze = Maze::from(vec![vec![0u32; context.cols]; context.rows]);

    let mut trace: Tracer = vec![];
    let mut tracer: Option<Tracer> = Some(vec![]);

    generate_maze(&mut maze, context.source, &context.procedure, &mut tracer);
    if let Some(maze_file_path) = context.maze_file_path.clone() {
        dump_maze_to_file(&maze_file_path, &maze);
    }

    let mut path: Vec<Node> = vec![];
    let mut last_millis = 0;
    let update_interval = 1000 / context.fps as u128;

    let mut trace_complete = false;
    let mut simulating = false;

    loop {
        if is_key_pressed(KeyCode::S) && !simulating && !trace_complete {
            println!("Starting Simulation");
            trace = tracer.clone().unwrap();
            simulating = true;
        }

        if is_key_pressed(KeyCode::Q) {
            break;
        }

        if simulating {
            if !trace_complete && last_millis + update_interval <= current_millis() {
                path = trace.remove(0);
                last_millis = current_millis();
                if trace.len() == 0 {
                    trace_complete = true;
                    simulating = false;
                }
            }

            draw_maze(
                draw_context,
                color_context,
                &visited,
                None,
                Some(&path),
                Some(context.source),
                None,
                true,
            );
        } else {
            if trace_complete {
                draw_maze(
                    draw_context,
                    color_context,
                    &maze,
                    None,
                    None,
                    Some(context.source),
                    None,
                    false,
                );
            } else {
                draw_maze(
                    draw_context,
                    color_context,
                    &visited,
                    None,
                    None,
                    Some(context.source),
                    None,
                    false,
                );
            }
        }

        next_frame().await
    }
}
