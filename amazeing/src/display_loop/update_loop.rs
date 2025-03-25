use crate::context::{ColorContext, DrawContext, ViewContext};
use crate::helper::{draw_maze, dump_maze_to_file, get_node_from_mouse_pos};
use macroquad::prelude::*;

pub(crate) async fn update_loop(
    context: &ViewContext,
    draw_context: &DrawContext,
    color_context: &ColorContext,
) {
    let maze = &mut context.maze.clone();

    loop {
        if is_key_pressed(KeyCode::Q) {
            dump_maze_to_file(&*context.maze_file_path, maze);
            break;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let value = if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                0
            } else {
                1
            };

            let node = get_node_from_mouse_pos(draw_context);

            maze[node] = value;
        }

        draw_maze(
            draw_context,
            color_context,
            maze,
            None,
            None,
            None,
            None,
            false,
        );
        next_frame().await
    }
}
