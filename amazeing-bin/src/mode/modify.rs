use crate::context::{DRAW_CTX, MOD_CTX};
use crate::display::action::quit_requested;
use crate::display::drawer::draw_maze;
use crate::helper::dumper::dump_maze_to_file;
use crate::helper::get_node_from_mouse_pos;
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;

async fn display_loop() {
    let maze = &mut MOD_CTX.read().unwrap().maze.clone();
    loop {
        if quit_requested() {
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

        draw_maze(maze);
        next_frame().await
    }
}

#[macroquad::main("Maze Editor")]
async fn main() {
    let (screen_width, screen_height) = DRAW_CTX.read().unwrap().screen_size();

    set_window_size(screen_width, screen_height + 30);

    display_loop().await;

    dump_maze_to_file(
        &*MOD_CTX.read().unwrap().maze_file_path,
        &MOD_CTX.read().unwrap().maze,
    );
}

pub(crate) fn modify() {
    main()
}
