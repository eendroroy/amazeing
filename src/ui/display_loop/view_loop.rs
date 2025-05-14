use crate::ui::component::scene::MazeScene;
use crate::ui::context::Colors;
use crate::ui::helper::current_millis;
use macroquad::prelude::*;

pub(crate) async fn view_loop(scene: MazeScene, color_context: &Colors) {
    loop {
        let current_frame_start_time = current_millis();

        clear_background(color_context.color_bg);

        scene.draw();

        if is_key_pressed(KeyCode::Q) {
            break;
        }

        if (is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl)) && is_key_pressed(KeyCode::I) {
            get_screen_data().export_png(&format!(
                "maze_{}_{}_{}.png",
                current_millis(),
                scene.maze.rows(),
                scene.maze.cols()
            ));
        }

        scene.delay_till_next_frame(current_frame_start_time);

        next_frame().await
    }
}
