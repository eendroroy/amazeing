use crate::_lib::tiled::{Node, Trace, Tracer};
use crate::command::ArgGenProcedure;
use crate::context::{ColorContext, CreateContext, DrawContext};
use crate::helper::{
    add_source, current_millis, delay_till_next_frame, draw_maze, dump_maze_to_file, generate_maze,
    generate_maze_tiles, populate_destination,
};
use macroquad::prelude::*;
use std::collections::HashMap;

pub(crate) async fn generate_simulation_loop(
    context: &CreateContext,
    draw_context: &DrawContext,
    color_context: &ColorContext,
) {
    let mut maze = generate_maze_tiles(context.rows, context.cols, draw_context);
    let mut traversed = maze.clone();
    let dummy_maze = maze.clone();

    let mut trace: Tracer = vec![];
    let mut tracer: Option<Tracer> = Some(vec![]);

    let mut path: Trace = HashMap::new();

    let mut trace_complete = false;
    let mut simulating = false;
    let mut paused = false;

    let sources = &mut vec![];
    let mut destination: Option<Node> = None;

    loop {
        let current_frame_start_time = current_millis();

        clear_background(color_context.color_bg);

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
                (sources, destination),
                true,
            );

            if is_key_pressed(KeyCode::Space) {
                paused = !paused;
            }
        } else if trace_complete {
            draw_maze(draw_context, color_context, &maze, None, None, (sources, destination), false);
        } else {
            draw_maze(draw_context, color_context, &traversed, None, None, (sources, destination), false);
        }

        if !simulating && !trace_complete {
            if is_mouse_button_released(MouseButton::Left) {
                add_source(draw_context, &maze, sources);
                if context.procedure == ArgGenProcedure::AStar {
                    populate_destination(draw_context, &maze, &mut destination)
                }
            }

            if (!sources.is_empty() && (is_key_pressed(KeyCode::G) || is_key_pressed(KeyCode::Space)))
                && (context.procedure != ArgGenProcedure::AStar || destination.is_some())
            {
                generate_maze(&mut maze, &draw_context.unit_shape, sources, destination, context, &mut tracer);
                if let Some(maze_file_path) = context.maze_file_path.clone() {
                    dump_maze_to_file(&maze_file_path, &maze);
                }
                trace = tracer.clone().unwrap();
                simulating = true;
            }
        }

        if (is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl)) && is_key_pressed(KeyCode::I) {
            get_screen_data().export_png(&format!("maze_{}_{}_{}.png", current_millis(), context.rows, context.cols));
        }

        if is_key_pressed(KeyCode::Q) {
            break;
        }

        delay_till_next_frame(current_frame_start_time, draw_context.fps as f32);

        next_frame().await
    }
}
