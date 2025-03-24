use crate::context::VIS_CTX;
use crate::helper::{draw_maze, dump_maze_to_file, get_node_from_mouse_pos};
use macroquad::prelude::*;

pub(crate) async fn update_loop() {
    let maze = &mut VIS_CTX.read().unwrap().maze.clone();

    loop {
        if is_key_pressed(KeyCode::Q) {
            dump_maze_to_file(&*VIS_CTX.read().unwrap().maze_file_path, maze);
            break;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let value = if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                0
            } else {
                1
            };

            let node = get_node_from_mouse_pos();

            maze[node] = value;
        }

        draw_maze(maze, None, None, None, None, false);
        next_frame().await
    }
}
