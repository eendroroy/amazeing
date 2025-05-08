use crate::command::ArgGenProcedure;
use crate::core::tiled::{Maze, Node, VOID};
use crate::ui::context::{ColorContext, CreateContext, DrawContext};
use crate::ui::helper::{
    convert_to_maze_shape, current_millis, delay_till_next_frame, dump_maze_to_file, generate_maze,
};
use crate::ui::shape::maze_mesh::MazeMesh;
use macroquad::prelude::*;

pub(crate) async fn generate_loop(
    shapes: &mut MazeMesh,
    maze: &mut Maze,
    context: &CreateContext,
    draw_context: &DrawContext,
    colors: &ColorContext,
) {
    let sources = &mut vec![];
    let mut destination: Option<Node> = None;
    let mut generated = false;

    loop {
        let current_frame_start_time = current_millis();

        clear_background(colors.color_bg);

        shapes.draw();

        if !generated && is_mouse_button_released(MouseButton::Left) {
            if let Some(node) = shapes.clicked_on(mouse_position()) {
                if maze[node] != VOID && !(is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift)) {
                    if sources.contains(&node) {
                        if let Some(index) = sources.iter().position(|value| *value == node) {
                            let node = sources.swap_remove(index);
                            shapes[node] = shapes.shape_factory.shape(node.row, node.col, colors.color_block)
                        }
                    } else {
                        sources.push(node);
                        shapes[node] = shapes.shape_factory.shape(node.row, node.col, colors.color_source)
                    }
                } else if maze[node] != VOID && (is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift)) {
                    if let Some(dest) = destination {
                        shapes[dest] = shapes.shape_factory.shape(dest.row, dest.col, colors.color_block)
                    }
                    destination = Some(node);
                    if let Some(dest) = destination {
                        shapes[dest] = shapes.shape_factory.shape(dest.row, dest.col, colors.color_destination)
                    }
                }
            }
        }

        if is_key_pressed(KeyCode::Q) {
            break;
        }

        if !generated
            && (!sources.is_empty() && (is_key_pressed(KeyCode::G) || is_key_pressed(KeyCode::Space)))
            && (context.procedure != ArgGenProcedure::AStar || destination.is_some())
        {
            generate_maze(maze, &draw_context.unit_shape, sources, destination, context, &mut None);
            *shapes = convert_to_maze_shape(&maze, &draw_context, &colors).1;
            sources
                .iter()
                .for_each(|node| shapes[*node] = shapes.shape_factory.shape(node.row, node.col, colors.color_source));
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
                    dump_maze_to_file(&maze_file_path, &maze);
                }
            }
        }

        delay_till_next_frame(current_frame_start_time, draw_context.fps as f32);

        next_frame().await
    }
}
