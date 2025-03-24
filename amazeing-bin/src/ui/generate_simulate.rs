use crate::context::GEN_CTX;
use crate::helper::{current_millis, draw_maze, dump_maze_to_file, generate_maze};
use amazeing::matrix::{Maze, Node, Tracer};
use macroquad::prelude::*;

pub(crate) async fn generate_simulation_loop() {
    let (rows, cols) = (GEN_CTX.read().unwrap().rows, GEN_CTX.read().unwrap().cols);
    let traversed = Maze::from(vec![vec![0u32; cols]; rows]);
    let mut maze = Maze::from(vec![vec![0u32; cols]; rows]);

    let mut trace: Tracer = vec![];
    let mut tracer: Option<Tracer> = Some(vec![]);

    generate_maze(
        &mut maze,
        GEN_CTX.read().unwrap().source,
        &GEN_CTX.read().unwrap().procedure,
        &mut tracer,
    );
    if let Some(maze_file_path) = GEN_CTX.read().unwrap().maze_file_path.clone() {
        dump_maze_to_file(&maze_file_path, &maze);
    }

    let mut current_path: Vec<Node> = vec![];
    let mut last_millis = 0;
    let update_interval = 1000 / GEN_CTX.read().unwrap().fps as u128;

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
                current_path = trace.remove(0);
                last_millis = current_millis();
                if trace.len() == 0 {
                    trace_complete = true;
                    simulating = false;
                }
            }

            draw_maze(
                &traversed,
                None,
                Some(&current_path),
                Some(GEN_CTX.read().unwrap().source),
                None,
                !trace_complete,
            );
        } else {
            if trace_complete {
                draw_maze(
                    &maze,
                    None,
                    None,
                    Some(GEN_CTX.read().unwrap().source),
                    None,
                    !trace_complete,
                );
            } else {
                draw_maze(
                    &traversed,
                    None,
                    None,
                    Some(GEN_CTX.read().unwrap().source),
                    None,
                    trace_complete,
                );
            }
        }

        next_frame().await
    }
}
