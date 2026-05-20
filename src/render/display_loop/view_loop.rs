use crate::render::helper::{current_micros, take_a_snap};
use crate::render::scene::MazeScene;
use macroquad::prelude::*;

pub(crate) async fn view_loop(scene: MazeScene) {
    loop {
        let current_frame_start_time = current_micros();
        scene.clear_and_draw();

        if is_key_pressed(KeyCode::Q) {
            break;
        }

        take_a_snap(&scene);

        scene.delay_till_next_frame(current_frame_start_time);

        next_frame().await
    }
}
