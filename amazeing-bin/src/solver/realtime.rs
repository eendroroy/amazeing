use crate::context::{DrawContext, COLORS, SOLVER_CONTEXT};
use crate::display::action::{populate_source_destination, quit_requested};
use crate::display::drawer::{draw_destination, draw_maze, draw_path, draw_source, get_conf};
use crate::helper::loader::loader_maze_from_file;
use crate::helper::run_algorithm;
use amazeing::maze::matrix::Maze;
use amazeing::DNode;
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;

pub(crate) async fn realtime_loop(
    maze: Maze,
    margin: f32,
    padding: f32,
    cell_width: f32,
    cell_height: f32,
) {
    let ctx = &DrawContext {
        margin,
        padding,
        cell_width,
        cell_height,
    };
    let mut current_path: Vec<DNode> = vec![];
    let mut from: Option<DNode> = None;
    let mut to: Option<DNode> = None;

    loop {
        clear_background(COLORS.color_bg);
        if is_mouse_button_pressed(MouseButton::Left) {
            populate_source_destination(&maze, ctx, &mut from, &mut to);

            if from.is_some() && to.is_some() {
                current_path = run_algorithm(&maze, from.unwrap(), to.unwrap());
            }
        }

        if quit_requested() {
            break;
        }

        draw_maze(&maze, ctx);
        draw_path(current_path.clone(), ctx);
        if from.is_some() {
            draw_source(from.unwrap(), ctx);
        }
        if to.is_some() {
            draw_destination(to.unwrap(), ctx);
        }
        next_frame().await
    }
}


#[macroquad::main(get_conf())]
pub async fn main() {
    let maze = SOLVER_CONTEXT.read().unwrap().maze.clone();

    let (margin, padding, cell_width, cell_height) = SOLVER_CONTEXT.read().unwrap().display_size();
    let (maze_width, maze_height) = (maze.cols(), maze.rows());
    let (screen_width, screen_height) = (
        margin + maze_width as f32 * (cell_width + padding) + margin,
        margin + maze_height as f32 * (cell_height + padding) + margin,
    );

    set_window_size(screen_width as u32, screen_height as u32 + 30);

    realtime_loop(maze, margin, padding, cell_width, cell_height).await
}

pub(crate) fn realtime(
    algorithm: String,
    heu: String,
    maze_file_path: String,
    fps: String,
    display_size: String,
) {
    SOLVER_CONTEXT.write().unwrap().title =
        String::from(format!("Realtime Maze Solver ({})", algorithm));
    SOLVER_CONTEXT.write().unwrap().display_size = display_size;

    if fps != String::from("") {
        SOLVER_CONTEXT.write().unwrap().fps = u8::from_str_radix(&fps, 10).unwrap();
    }
    if heu != String::from("") {
        SOLVER_CONTEXT.write().unwrap().heu = heu;
    }

    let maze = loader_maze_from_file(&*maze_file_path);
    SOLVER_CONTEXT.write().unwrap().maze = maze.clone();

    main();
}
