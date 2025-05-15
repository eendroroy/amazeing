use crate::core::tiled::{BLOCK, OPEN};
use crate::ui::component::scene::MazeScene;
use crate::ui::helper::{current_millis, save_maze, take_a_snap};
use macroquad::prelude::*;

pub(crate) async fn update_loop(scene: &mut MazeScene) {
    loop {
        let current_frame_start_time = current_millis();

        scene.clear_and_draw();

        if is_mouse_button_pressed(MouseButton::Left) {
            let (value, color) = if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                (BLOCK, scene.colors.color_block)
            } else {
                (OPEN, scene.colors.color_open)
            };

            if let Some(node) = scene.clicked_on(mouse_position()) {
                scene.update_node(node, value, color);
            }
        }

        take_a_snap(scene);
        save_maze(scene);

        if is_key_pressed(KeyCode::Q) {
            break;
        }

        scene.delay_till_next_frame(current_frame_start_time);

        next_frame().await
    }
}
