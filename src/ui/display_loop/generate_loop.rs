use crate::command::ArgGenProcedure;
use crate::core::tiled::{Node, VOID};
use crate::ui::component::scene::MazeScene;
use crate::ui::context::{AmazeingContext, Colors, DrawContext};
use crate::ui::helper::{current_millis, delay_till_next_frame, dump_maze_to_file, generate_maze};
use macroquad::prelude::*;

pub(crate) async fn generate_loop(
    scene: &mut MazeScene,
    context: &AmazeingContext,
    draw_context: &DrawContext,
    colors: &Colors,
) {
    let sources = &mut vec![];
    let mut destination: Option<Node> = None;
    let mut generated = false;

    loop {
        let current_frame_start_time = current_millis();

        clear_background(colors.color_bg);

        scene.draw();
        scene.draw_bound();

        if !generated && is_mouse_button_released(MouseButton::Left) {
            if let Some(node) = scene.clicked_on(mouse_position()) {
                if scene.maze[node] != VOID && !(is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift)) {
                    if sources.contains(&node) {
                        if let Some(index) = sources.iter().position(|value| *value == node) {
                            let node = sources.swap_remove(index);
                            scene.update_color(node, colors.color_block);
                        }
                    } else {
                        sources.push(node);
                        scene.update_color(node, colors.color_source);
                    }
                } else if scene.maze[node] != VOID
                    && (is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift))
                {
                    if let Some(dest) = destination {
                        scene.update_color(dest, colors.color_block);
                    }
                    destination = Some(node);
                    if let Some(dest) = destination {
                        scene.update_color(dest, colors.color_destination);
                    }
                }
            }
        }

        if is_key_pressed(KeyCode::Q) {
            break;
        }

        if !generated
            && (!sources.is_empty() && (is_key_pressed(KeyCode::G) || is_key_pressed(KeyCode::Space)))
            && (context.generation_procedure != ArgGenProcedure::AStar || destination.is_some())
        {
            generate_maze(&mut scene.maze, &draw_context.unit_shape, sources, destination, context, &mut None);
            scene.update();
            sources
                .iter()
                .for_each(|node| scene.update_color(*node, colors.color_source));
            generated = true;
        }

        if is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl) {
            if is_key_pressed(KeyCode::I) {
                get_screen_data().export_png(&format!(
                    "maze_{}_{}_{}.png",
                    current_millis(),
                    context.rows,
                    context.cols
                ));
            }

            if is_key_pressed(KeyCode::S) {
                if let Some(maze_file_path) = context.maze_file_path.clone() {
                    dump_maze_to_file(&maze_file_path, &scene.maze);
                }
            }
        }

        delay_till_next_frame(current_frame_start_time, draw_context.fps as f32);

        next_frame().await
    }
}
