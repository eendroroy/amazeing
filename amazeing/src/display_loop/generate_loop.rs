use crate::context::{ColorContext, CreateContext, DrawContext};
use crate::helper::{current_millis, delay_till_next_frame, draw_maze, dump_maze_to_file, generate_maze};
use amazeing::matrix::Maze;
use macroquad::prelude::*;

pub(crate) async fn generate_loop(context: &CreateContext, draw_context: &DrawContext, color_context: &ColorContext) {
    let mut maze = Maze::from(vec![vec![0u32; context.cols]; context.rows]);

    loop {
        let current_frame_start_time = current_millis();

        clear_background(color_context.color_bg);

        draw_maze(draw_context, color_context, &maze, None, None, (context.sources.clone(), None), false);

        if is_key_pressed(KeyCode::Q) {
            break;
        }

        if is_key_pressed(KeyCode::G) || is_key_pressed(KeyCode::Space) {
            maze = Maze::from(vec![vec![0u32; context.cols]; context.rows]);
            generate_maze(&mut maze, &draw_context.u_shape, context.sources.clone(), &context.procedure, &mut None);
        }

        if (is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl)) && is_key_pressed(KeyCode::I) {
            get_screen_data().export_png(&format!("maze_{}_{}_{}.png", current_millis(), context.rows, context.cols));
        }

        delay_till_next_frame(current_frame_start_time, draw_context.fps as f32);

        next_frame().await
    }

    if let Some(maze_file_path) = context.maze_file_path.clone() {
        dump_maze_to_file(&maze_file_path, &maze);
    }
}
