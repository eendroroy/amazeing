use crate::context::{DRAW_CTX, VIS_CTX};
use crate::display::action::quit_requested;
use crate::display::drawer::draw_maze;
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;

async fn looper() {
    loop {
        if quit_requested() {
            break;
        }

        draw_maze(&VIS_CTX.read().unwrap().maze);
        next_frame().await
    }
}

#[macroquad::main("Maze Viewer")]
async fn main() {
    let (screen_width, screen_height) = DRAW_CTX.read().unwrap().screen_size();

    set_window_size(screen_width as u32, screen_height as u32 + 30);

    looper().await
}

pub(crate) fn visualize() {
    main()
}
