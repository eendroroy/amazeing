use crate::context::{ColorContext, CreateContext, DrawContext};
use crate::helper::{draw_maze, dump_maze_to_file, generate_maze};
use amazeing::matrix::Maze;
use macroquad::prelude::*;

pub(crate) async fn generate_loop(
    context: &CreateContext,
    draw_context: &DrawContext,
    color_context: &ColorContext,
) {
    let mut maze = Maze::from(vec![vec![0u32; context.cols]; context.rows]);

    generate_maze(&mut maze, context.source, &context.procedure, &mut None);

    if let Some(maze_file_path) = context.maze_file_path.clone() {
        dump_maze_to_file(&maze_file_path, &maze);
    }

    loop {
        if is_key_pressed(KeyCode::Q) {
            break;
        }

        draw_maze(
            draw_context,
            color_context,
            &maze,
            None,
            None,
            None,
            None,
            false,
        );
        next_frame().await
    }
}
