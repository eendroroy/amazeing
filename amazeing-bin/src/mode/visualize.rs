use crate::context::{DRAW_CTX, VIS_CTX};
use crate::display::action::quit_requested;
use crate::display::drawer::draw_maze;
use crate::helper::loader::loader_maze_from_file;
use amazeing::maze::matrix::Maze;
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;

async fn looper(maze: &mut Maze) {
    loop {
        if quit_requested() {
            break;
        }

        draw_maze(maze);
        next_frame().await
    }
}

#[macroquad::main("Maze Viewer")]
async fn main() {
    let mut maze = loader_maze_from_file(&*VIS_CTX.read().unwrap().maze_file_path);

    let (screen_width, screen_height) = DRAW_CTX.read().unwrap().screen_size();

    set_window_size(screen_width as u32, screen_height as u32 + 30);

    looper(&mut maze).await
}

pub(crate) fn visualize() {
    main()
}
