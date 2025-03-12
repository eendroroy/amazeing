use crate::context::{DRAW_CTX, SIM_CTX};
use crate::display::action::quit_requested;
use crate::display::drawer::{
    draw_current_path, draw_maze, draw_path, draw_source_destination, draw_traversed,
};
use crate::helper::{current_millis, run_algorithm};
use amazeing::maze::matrix::Maze;
use amazeing::DNode;
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;

async fn display_loop() {
    let maze = &SIM_CTX.read().unwrap().maze;
    let trace = &mut SIM_CTX.read().unwrap().tracer.clone();
    let mut current_path: Vec<DNode> = vec![];
    let mut last_millis = current_millis();
    let update_interval = 1000 / SIM_CTX.read().unwrap().fps as u128;

    let mut traversed: Maze = Maze::from(vec![vec![0u32; maze.cols()]; maze.rows()]);

    let mut trace_complete = false;
    let mut simulating = false;

    loop {
        if is_key_pressed(KeyCode::S) && !simulating {
            println!("Starting Simulation");
            simulating = true;
            current_path = trace.remove(0);
            last_millis = current_millis();
            if trace.len() == 0 {
                trace_complete = true;
            }
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
        draw_source_destination(
            SIM_CTX.read().unwrap().source,
            SIM_CTX.read().unwrap().destination,
        );
        next_frame().await
    }
}

#[macroquad::main("Maze Simulation")]
async fn main() {
    let (screen_width, screen_height) = DRAW_CTX.read().unwrap().screen_size();

    set_window_size(screen_width as u32, screen_height as u32 + 30);

    display_loop().await
}

pub(crate) fn simulate() {
    let mut tracer: Option<Vec<Vec<DNode>>> = Some(vec![]);

    run_algorithm(
        &SIM_CTX.read().unwrap().maze,
        SIM_CTX.read().unwrap().source,
        SIM_CTX.read().unwrap().destination,
        SIM_CTX.read().unwrap().proc.clone(),
        Some(SIM_CTX.read().unwrap().heuristic.clone()),
        &mut tracer,
    );

    SIM_CTX.write().unwrap().tracer = tracer.unwrap();

    main();
}
