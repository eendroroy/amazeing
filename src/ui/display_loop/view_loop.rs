use crate::ui::component::scene::MazeScene;
use crate::ui::helper::{current_millis, take_a_snap};
use macroquad::prelude::*;

pub(crate) async fn view_loop(scene: MazeScene) {
    loop {
        let current_frame_start_time = current_millis();
        scene.clear_and_draw();

        if is_key_pressed(KeyCode::Q) {
            break;
        }

        take_a_snap(&scene);

        scene.delay_till_next_frame(current_frame_start_time);

        next_frame().await
    }
}
