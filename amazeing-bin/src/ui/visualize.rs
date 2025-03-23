use crate::context::{DRAW_CTX, VIS_CTX};
use crate::helper::drawer::draw_maze;
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;

async fn looper() {
    loop {
        if is_key_pressed(KeyCode::Q) {
            break;
        }

        draw_maze(&VIS_CTX.read().unwrap().maze);
        next_frame().await
    }
}

#[macroquad::main("View Maze")]
pub async fn main() {
    let (screen_width, screen_height) = DRAW_CTX.read().unwrap().screen_size();

    set_window_size(screen_width, screen_height + 30);

    looper().await
}
