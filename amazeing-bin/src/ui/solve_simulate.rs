use crate::context::{DRAW_CTX, SOLVE_CTX};
use crate::helper::drawer::{
    draw_current_path, draw_destination, draw_maze, draw_path, draw_source, draw_traversed,
};
use crate::helper::{current_millis, populate_source_destination, solve_maze};
use amazeing::matrix::{Maze, Node, Tracer};
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;

async fn display_loop(maze: &Maze) {
    let mut trace: Tracer = vec![];
    let mut tracer: Option<Tracer> = Some(vec![]);
    let mut current_path: Vec<Node> = vec![];
    let mut last_millis = current_millis();
    let update_interval = 1000 / SOLVE_CTX.read().unwrap().fps as u128;

    let mut traversed: Maze = Maze::from(vec![vec![0u32; maze.cols()]; maze.rows()]);

    let mut trace_complete = false;
    let mut simulating = false;

    let mut source: Option<Node> = None;
    let mut destination: Option<Node> = None;

    loop {
        if is_mouse_button_pressed(MouseButton::Left) && !simulating {
            populate_source_destination(&maze, &mut source, &mut destination);
        }

        if is_key_pressed(KeyCode::S) && !simulating && source.is_some() && destination.is_some() {
            println!("Starting Simulation");
            solve_maze(
                &maze,
                source.unwrap(),
                destination.unwrap(),
                &SOLVE_CTX.read().unwrap().proc.clone(),
                Some(SOLVE_CTX.read().unwrap().heuristic.clone()),
                &mut tracer,
            );
            simulating = true;
            trace = tracer.clone().unwrap();
            current_path = trace.remove(0);
            last_millis = current_millis();
            if trace.len() == 0 {
                trace_complete = true;
            }
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
                }
            }

            draw_maze(&maze);
            draw_traversed(&traversed);
            if trace_complete {
                draw_path(current_path.clone());
            } else {
                draw_current_path(current_path.clone(), &mut traversed);
            };
        } else {
            draw_maze(&maze);
        }

        if source.is_some() {
            draw_source(source.unwrap());
        }
        if destination.is_some() {
            draw_destination(destination.unwrap());
        }
        next_frame().await
    }
}

#[macroquad::main("Simulate Maze Solution")]
pub async fn main() {
    let maze = &SOLVE_CTX.read().unwrap().maze;
    let (screen_width, screen_height) = DRAW_CTX.read().unwrap().screen_size();

    set_window_size(screen_width, screen_height + 30);

    display_loop(maze).await
}
