use crate::core::tiled::{Node, OPEN};
use crate::ui::component::scene::MazeScene;
use crate::ui::context::{AmazeingContext, Colors};
use crate::ui::helper::{current_millis, solve_maze};
use macroquad::prelude::*;

pub(crate) async fn solve_loop(scene: &mut MazeScene, context: &AmazeingContext, colors: &Colors) {
    let sources = &mut vec![];
    let mut destination: Option<Node> = None;
    let mut path: Vec<Node> = vec![];

    loop {
        let current_frame_start_time = current_millis();

        clear_background(colors.color_bg);

        scene.draw();

        if is_mouse_button_pressed(MouseButton::Left) {
            if let Some(node) = scene.clicked_on(mouse_position()) {
                if scene.maze[node] == OPEN {
                    if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                        if let Some(node) = destination {
                            scene.update_color(node, colors.color_open)
                        }
                        destination = Some(node);
                        scene.update_color(node, colors.color_destination)
                    } else {
                        if let Some(node) = sources.first() {
                            scene.update_color(*node, colors.color_open)
                        }
                        *sources = vec![node];
                        scene.update_color(node, colors.color_source)
                    }
                }
            }

            if !sources.is_empty() && destination.is_some() {
                path.iter().for_each(|node| {
                    if sources.first().unwrap().ne(node) && destination.unwrap().ne(node) {
                        scene.update_color(*node, colors.color_open)
                    }
                });

                path = solve_maze(
                    &scene.maze,
                    *sources.first().unwrap(),
                    destination.unwrap(),
                    &context.solve_procedure,
                    context.heuristic,
                    &mut None,
                );

                path.iter().for_each(|node| {
                    if sources.first().unwrap().ne(node) && destination.unwrap().ne(node) {
                        scene.update_color(*node, colors.color_path)
                    }
                })
            }
        }

        if (is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl)) && is_key_pressed(KeyCode::I) {
            get_screen_data().export_png(&format!(
                "maze_{}_{}_{}.png",
                current_millis(),
                scene.maze.rows(),
                scene.maze.cols()
            ));
        }

        if is_key_pressed(KeyCode::Q) {
            break;
        }

        scene.delay_till_next_frame(current_frame_start_time);

        next_frame().await
    }
}
