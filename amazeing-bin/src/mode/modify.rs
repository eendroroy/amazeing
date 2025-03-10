use crate::context::CONTEXT;
use crate::display::action::quit_requested;
use crate::display::drawer::draw_maze;
use crate::helper::dumper::dump_maze_to_file;
use crate::helper::get_node_from_mouse_pos;
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;

async fn display_loop() {
    let maze = &mut CONTEXT.read().unwrap().maze.clone();
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

            let node = get_node_from_mouse_pos(&CONTEXT.read().unwrap().draw_context());

            maze[node] = value;
        }

        draw_maze(maze, &CONTEXT.read().unwrap().draw_context());
        next_frame().await
    }
}

#[macroquad::main("Maze Editor")]
async fn main() {
    let (screen_width, screen_height) = CONTEXT.read().unwrap().screen_size();

    set_window_size(screen_width as u32, screen_height as u32 + 30);

    display_loop().await;

    dump_maze_to_file(
        &*CONTEXT.read().unwrap().maze_file_path,
        &CONTEXT.read().unwrap().maze,
    );
}

pub(crate) fn modify() {
    main()
}
