use crate::context::{DrawContext, CONTEXT};
use crate::display::action::quit_requested;
use crate::display::drawer::draw_maze;
use crate::helper::get_node_from_mouse_pos;
use crate::helper::loader::loader_maze_from_file;
use amazeing::maze::matrix::Maze;
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;

async fn looper(maze: &mut Maze, margin: f32, padding: f32, cell_width: f32, cell_height: f32) {
    loop {
        if quit_requested() {
            break;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let (r, c) = get_node_from_mouse_pos(&CONTEXT.read().unwrap().draw_context());

            println!("{},{}", r, c);
        }

        draw_maze(
            maze,
            &DrawContext {
                margin,
                padding,
                cell_width,
                cell_height,
            },
        );
        next_frame().await
    }
}

#[macroquad::main("Maze View")]
async fn main() {
    let mut maze = loader_maze_from_file(&*CONTEXT.read().unwrap().maze_file_path);

    let (margin, padding, cell_width, cell_height) = CONTEXT.read().unwrap().display_size();

    let (maze_width, maze_height) = (maze.cols(), maze.rows());
    let (screen_width, screen_height) = (
        margin + maze_width as f32 * (cell_width + padding) + margin,
        margin + maze_height as f32 * (cell_width + padding) + margin,
    );

    set_window_size(screen_width as u32, screen_height as u32 + 30);

    looper(&mut maze, margin, padding, cell_width, cell_height).await
}

pub(crate) fn visualize() {
    main()
}
