use crate::context::VIS_CTX;
use crate::helper::draw_maze;
use macroquad::prelude::*;

pub(crate) async fn view_loop() {
    loop {
        if is_key_pressed(KeyCode::Q) {
            break;
        }

        draw_maze(&VIS_CTX.read().unwrap().maze, None, None, None, None, false);
        next_frame().await
    }
}
