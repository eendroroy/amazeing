use crate::context::CONTEXT;
use crate::display::action::{populate_source_destination, quit_requested};
use crate::display::drawer::{draw_destination, draw_maze, draw_path, draw_source, get_conf};
use crate::helper::loader::loader_maze_from_file;
use crate::helper::run_algorithm;
use amazeing::DNode;
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;

async fn display_loop() {
    let maze = &CONTEXT.read().unwrap().maze;
    let dc = &CONTEXT.read().unwrap().draw_context();
    let mut current_path: Vec<DNode> = vec![];
    let mut from: Option<DNode> = None;
    let mut to: Option<DNode> = None;

    loop {
        clear_background(CONTEXT.read().unwrap().colors.color_bg);
        if is_mouse_button_pressed(MouseButton::Left) {
            populate_source_destination(&maze, dc, &mut from, &mut to);

            if from.is_some() && to.is_some() {
                current_path = run_algorithm(&maze, from.unwrap(), to.unwrap(), &mut None);
            }
        }

        if quit_requested() {
            break;
        }

        draw_maze(&maze, dc);
        draw_path(current_path.clone(), dc);
        if from.is_some() {
            draw_source(from.unwrap(), dc);
        }
        if to.is_some() {
            draw_destination(to.unwrap(), dc);
        }
        next_frame().await
    }
}


#[macroquad::main(get_conf())]
async fn main() {
    let (screen_width, screen_height) = CONTEXT.read().unwrap().screen_size();
    set_window_size(screen_width as u32, screen_height as u32 + 30);

    display_loop().await
}

pub(crate) fn realtime() {
    let maze = loader_maze_from_file(&*CONTEXT.read().unwrap().maze_file_path);
    CONTEXT.write().unwrap().maze = maze.clone();

    main();
}
