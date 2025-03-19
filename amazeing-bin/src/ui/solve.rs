use crate::context::{COLOR_CTX, DRAW_CTX, SOLVE_CTX};
use crate::display::action::{populate_source_destination, quit_requested};
use crate::display::drawer::{draw_destination, draw_maze, draw_path, draw_source};
use crate::helper::solve_maze;
use amazeing::matrix::Node;
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;

async fn display_loop() {
    let maze = &SOLVE_CTX.read().unwrap().maze;
    let mut current_path: Vec<Node> = vec![];
    let mut from: Option<Node> = None;
    let mut to: Option<Node> = None;

    loop {
        clear_background(COLOR_CTX.read().unwrap().color_bg);
        if is_mouse_button_pressed(MouseButton::Left) {
            populate_source_destination(&maze, &mut from, &mut to);

            if from.is_some() && to.is_some() {
                current_path = solve_maze(
                    &maze,
                    from.unwrap(),
                    to.unwrap(),
                    &SOLVE_CTX.read().unwrap().proc,
                    Some(SOLVE_CTX.read().unwrap().heuristic.clone()),
                    &mut None,
                );
            }
        }

        if quit_requested() {
            break;
        }

        draw_maze(&maze);
        draw_path(current_path.clone());
        if from.is_some() {
            draw_source(from.unwrap());
        }
        if to.is_some() {
            draw_destination(to.unwrap());
        }
        next_frame().await
    }
}

#[macroquad::main("Solve Maze")]
pub async fn main() {
    let (screen_width, screen_height) = DRAW_CTX.read().unwrap().screen_size();
    set_window_size(screen_width, screen_height + 30);

    display_loop().await
}
