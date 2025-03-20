use crate::context::{DRAW_CTX, GEN_CTX};
use crate::display::action::quit_requested;
use crate::display::drawer::{draw_current_path, draw_maze, draw_path, draw_source};
use crate::helper::{current_millis, dump_maze_to_file, generate_maze};
use amazeing::matrix::{Maze, Node, Tracer};
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;

async fn display_loop(rows: usize, cols: usize) {
    let mut traversed = Maze::from(vec![vec![0u32; cols]; rows]);
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

        if quit_requested() {
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

            draw_maze(&traversed);
            if trace_complete {
                draw_path(current_path.clone());
            } else {
                draw_current_path(current_path.clone(), &mut traversed);
            };
        } else {
            if trace_complete {
                draw_maze(&maze);
            } else {
                draw_maze(&traversed);
            }
        }

        draw_source(GEN_CTX.read().unwrap().source);
        next_frame().await
    }
}

#[macroquad::main("Simulate Maze Generation")]
pub async fn main() {
    let (screen_width, screen_height) = DRAW_CTX.read().unwrap().screen_size();

    set_window_size(screen_width, screen_height + 30);

    display_loop(GEN_CTX.read().unwrap().rows, GEN_CTX.read().unwrap().cols).await
}
