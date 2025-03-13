use crate::context::{COL_CTX, DRAW_CTX, REL_CTX};
use crate::display::action::{populate_source_destination, quit_requested};
use crate::display::drawer::{draw_destination, draw_maze, draw_path, draw_source};
use crate::helper::run_algorithm;
use amazeing::DNode;
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;

async fn display_loop() {
    let maze = &REL_CTX.read().unwrap().maze;
    let mut current_path: Vec<DNode> = vec![];
    let mut from: Option<DNode> = None;
    let mut to: Option<DNode> = None;

    loop {
        clear_background(COL_CTX.read().unwrap().color_bg);
        if is_mouse_button_pressed(MouseButton::Left) {
            populate_source_destination(&maze, &mut from, &mut to);

            if from.is_some() && to.is_some() {
                current_path = run_algorithm(
                    &maze,
                    from.unwrap(),
                    to.unwrap(),
                    REL_CTX.read().unwrap().proc.clone(),
                    Some(REL_CTX.read().unwrap().heuristic.clone()),
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

#[macroquad::main("Realtime Path Finder")]
async fn main() {
    let (screen_width, screen_height) = DRAW_CTX.read().unwrap().screen_size();
    set_window_size(screen_width as u32, screen_height as u32 + 30);

    display_loop().await
}

pub(crate) fn solve() {
    main();
}
