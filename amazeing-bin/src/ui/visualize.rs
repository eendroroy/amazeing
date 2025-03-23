use crate::context::{DRAW_CTX, VIS_CTX};
use crate::helper::draw_maze;
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;

#[macroquad::main("View Maze")]
pub async fn main() {
    let (screen_width, screen_height) = DRAW_CTX.read().unwrap().screen_size();

    set_window_size(screen_width, screen_height + 30);

    loop {
        if is_key_pressed(KeyCode::Q) {
            break;
        }

        draw_maze(&VIS_CTX.read().unwrap().maze, None, None, None, None, false);
        next_frame().await
    }
}
