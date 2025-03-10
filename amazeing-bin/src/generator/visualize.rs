use crate::command::loader::loader_maze_from_file;
use crate::context::{DrawContext, GENERATOR_CONTEXT};
use crate::display::drawer::draw_maze;
use amazeing::maze::matrix::Maze;
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;

async fn looper(maze: &mut Maze, margin: f32, padding: f32, cell_width: f32, cell_height: f32) {
    loop {
        if is_key_pressed(KeyCode::Q) {
            break;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let r = ((my - margin) / (cell_height + padding)).floor();
            let c = ((mx - margin) / (cell_width + padding)).floor();

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
pub async fn main() {
    let mut maze = loader_maze_from_file(&*GENERATOR_CONTEXT.read().unwrap().maze_file_path);

    let (margin, padding, cell_width, cell_height) =
        GENERATOR_CONTEXT.read().unwrap().display_size();

    let (maze_width, maze_height) = (maze.cols(), maze.rows());
    let (screen_width, screen_height) = (
        margin + maze_width as f32 * (cell_width + padding) + margin,
        margin + maze_height as f32 * (cell_width + padding) + margin,
    );

    set_window_size(screen_width as u32, screen_height as u32 + 30);

    looper(&mut maze, margin, padding, cell_width, cell_height).await
}
