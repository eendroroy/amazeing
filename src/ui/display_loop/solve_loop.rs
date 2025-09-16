use crate::core::tiled::Node;
use crate::ui::component::scene::MazeScene;
use crate::ui::helper::{current_millis, handle_mouse_click, solve_maze, take_a_snap};
use macroquad::prelude::*;

pub(crate) async fn solve_loop(scene: &mut MazeScene) {
    let sources = &mut vec![];
    let mut destination: Option<Node> = None;
    let mut path: Vec<Node> = vec![];

    loop {
        let current_frame_start_time = current_millis();

        scene.clear_and_draw();

        if is_mouse_button_pressed(MouseButton::Left) {
            handle_mouse_click(scene, sources, &mut destination, mouse_position());

            if !sources.is_empty()
                && let Some(destination) = destination
            {
                path.iter().for_each(|node| {
                    if sources.first().unwrap().ne(node) && destination.ne(node) {
                        scene.display_open(*node)
                    }
                });

                path = solve_maze(
                    &scene.maze,
                    *sources.first().unwrap(),
                    destination,
                    &scene.context.solve_procedure,
                    scene.context.heuristic,
                    &mut None,
                );

                path.iter().for_each(|node| {
                    if sources.first().unwrap().ne(node) && destination.ne(node) {
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
