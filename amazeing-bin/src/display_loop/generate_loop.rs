use crate::context::{GEN_CTX, VIS_CTX};
use crate::helper::{draw_maze, dump_maze_to_file, generate_maze};
use macroquad::prelude::*;
use amazeing::matrix::Maze;

pub(crate) async fn generate_loop() {
    let (rows, cols) = (GEN_CTX.read().unwrap().rows, GEN_CTX.read().unwrap().cols);

    let mut maze = Maze::from(vec![vec![0u32; cols]; rows]);

    generate_maze(
        &mut maze,
        GEN_CTX.read().unwrap().source,
        &GEN_CTX.read().unwrap().procedure,
        &mut None,
    );

    if let Some(maze_file_path) = GEN_CTX.read().unwrap().maze_file_path.clone() {
        dump_maze_to_file(&maze_file_path, &maze);
    }

    loop {
        if is_key_pressed(KeyCode::Q) {
            break;
        }

        draw_maze(&VIS_CTX.read().unwrap().maze, None, None, None, None, false);
        next_frame().await
    }
}
