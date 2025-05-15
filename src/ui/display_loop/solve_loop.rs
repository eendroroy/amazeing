use crate::core::tiled::{Node, OPEN};
use crate::ui::component::scene::MazeScene;
use crate::ui::helper::{current_millis, solve_maze, take_a_snap};
use macroquad::prelude::*;

pub(crate) async fn solve_loop(scene: &mut MazeScene) {
    let sources = &mut vec![];
    let mut destination: Option<Node> = None;
    let mut path: Vec<Node> = vec![];

    loop {
        let current_frame_start_time = current_millis();

        scene.clear_and_draw();

        if is_mouse_button_pressed(MouseButton::Left) {
            if let Some(node) = scene.clicked_on(mouse_position()) {
                if scene.maze[node] == OPEN {
                    if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                        if let Some(node) = destination {
                            scene.display_open(node)
                        }
                        destination = Some(node);
                        scene.display_destination(node)
                    } else {
                        if let Some(node) = sources.first() {
                            scene.display_open(*node)
                        }
                        *sources = vec![node];
                        scene.display_source(node)
                    }
                }
            }

            if !sources.is_empty() && destination.is_some() {
                path.iter().for_each(|node| {
                    if sources.first().unwrap().ne(node) && destination.unwrap().ne(node) {
                        scene.display_open(*node)
                    }
                });

                path = solve_maze(
                    &scene.maze,
                    *sources.first().unwrap(),
                    destination.unwrap(),
                    &scene.context.solve_procedure,
                    scene.context.heuristic,
                    &mut None,
                );

                path.iter().for_each(|node| {
                    if sources.first().unwrap().ne(node) && destination.unwrap().ne(node) {
                        scene.display_path(*node)
                    }
                })
            }
        }

        take_a_snap(scene);

        if is_key_pressed(KeyCode::Q) {
            break;
        }

        scene.delay_till_next_frame(current_frame_start_time);

        next_frame().await
    }
}
