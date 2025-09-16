use crate::command::ArgProcedure;
use crate::core::tiled::{Node, VOID};
use crate::ui::component::scene::MazeScene;
use crate::ui::helper::{current_millis, generate_maze, save_maze, take_a_snap};
use macroquad::prelude::*;

pub(crate) async fn generate_loop(scene: &mut MazeScene) {
    let sources = &mut vec![];
    let mut destination: Option<Node> = None;
    let mut generated = false;

    loop {
        let current_frame_start_time = current_millis();

        scene.clear_and_draw();

        if !generated && is_mouse_button_released(MouseButton::Left) {
            if let Some(node) = scene.clicked_on(mouse_position()) {
                if scene.maze[node] != VOID && !(is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift)) {
                    if sources.contains(&node) {
                        if let Some(index) = sources.iter().position(|value| *value == node) {
                            let node = sources.swap_remove(index);
                            scene.display_block(node);
                        }
                    } else {
                        sources.push(node);
                        scene.display_source(node);
                    }
                } else if scene.maze[node] != VOID
                    && (is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift))
                {
                    if let Some(dest) = destination {
                        scene.display_block(dest);
                    }
                    destination = Some(node);
                    if let Some(dest) = destination {
                        scene.display_destination(dest);
                    }
                }
            }
        }

        if is_key_pressed(KeyCode::Q) {
            break;
        }

        if !generated
            && (!sources.is_empty() && (is_key_pressed(KeyCode::G) || is_key_pressed(KeyCode::Space)))
            && (scene.context.procedure != ArgProcedure::AStar || destination.is_some())
        {
            generate_maze(&mut scene.maze, sources, destination, &scene.context, &mut None);
            scene.update();
            sources.iter().for_each(|node| scene.display_source(*node));
            generated = true;
        }

        take_a_snap(scene);
        save_maze(scene);

        scene.delay_till_next_frame(current_frame_start_time);

        next_frame().await
    }
}
