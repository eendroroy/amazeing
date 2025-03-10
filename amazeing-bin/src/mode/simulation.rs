use crate::context::CONTEXT;
use crate::display::action::quit_requested;
use crate::display::drawer::{
    draw_current_path, draw_maze, draw_path, draw_source_destination, draw_traversed, get_conf,
};
use crate::helper::{current_millis, run_algorithm};
use amazeing::maze::matrix::Maze;
use amazeing::DNode;
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;

async fn display_loop() {
    let dc = &CONTEXT.read().unwrap().draw_context();
    let maze = &CONTEXT.read().unwrap().maze;
    let trace = &mut CONTEXT.read().unwrap().tracer.clone();
    let mut current_path: Vec<DNode> = vec![];
    let mut last_millis = current_millis();
    let update_interval = 1000 / CONTEXT.read().unwrap().fps as u128;

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

            draw_maze(&maze, dc);
            draw_traversed(&traversed, dc);
            if trace_complete {
                draw_path(current_path.clone(), dc);
            } else {
                draw_current_path(current_path.clone(), dc, &mut traversed);
            };
        } else {
            draw_maze(&maze, dc);
        }
        draw_source_destination(
            CONTEXT.read().unwrap().source,
            CONTEXT.read().unwrap().destination,
            dc,
        );
        next_frame().await
    }
}

#[macroquad::main(get_conf())]
async fn main() {
    let (screen_width, screen_height) = CONTEXT.read().unwrap().screen_size();

    set_window_size(screen_width as u32, screen_height as u32 + 30);

    display_loop().await
}

pub(crate) fn simulate() {
    let mut tracer: Option<Vec<Vec<DNode>>> = Some(vec![]);

    run_algorithm(
        &CONTEXT.read().unwrap().maze,
        CONTEXT.read().unwrap().source,
        CONTEXT.read().unwrap().destination,
        &mut tracer,
    );

    CONTEXT.write().unwrap().tracer = tracer.unwrap();

    main();
}
