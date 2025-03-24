use crate::context::{ColorContext, DrawContext, SolveContext};
use crate::helper::{current_millis, draw_maze, populate_source_destination, solve_maze};
use amazeing::matrix::{Maze, Node, Tracer};
use macroquad::prelude::*;

pub(crate) async fn solve_simulation_loop(
    context: SolveContext,
    draw_context: &DrawContext,
    color_context: &ColorContext,
) {
    let maze = &context.maze;
    let mut trace: Tracer = vec![];
    let mut tracer: Option<Tracer> = Some(vec![]);
    let mut current_path: Vec<Node> = vec![];
    let mut last_millis = current_millis();
    let update_interval = 1000 / context.fps as u128;

    let mut traversed: Maze = Maze::from(vec![vec![0u32; maze.cols()]; maze.rows()]);

    let mut trace_complete = false;
    let mut simulating = false;

    let mut source: Option<Node> = None;
    let mut destination: Option<Node> = None;

    loop {
        if is_mouse_button_pressed(MouseButton::Left) && !simulating {
            populate_source_destination(draw_context, &maze, &mut source, &mut destination);
        }

        if is_key_pressed(KeyCode::S) && !simulating && source.is_some() && destination.is_some() {
            println!("Starting Simulation");
            solve_maze(
                &maze,
                source.unwrap(),
                destination.unwrap(),
                &context.procedure.clone(),
                Some(context.heuristic.clone()),
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

            draw_maze(
                draw_context,
                color_context,
                &maze,
                Some(&mut traversed),
                Some(&current_path),
                source,
                destination,
                !trace_complete,
            )
        } else {
            draw_maze(
                draw_context,
                color_context,
                &maze,
                None,
                None,
                source,
                destination,
                false,
            );
        }

        next_frame().await
    }
}
