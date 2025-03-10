use crate::command::loader::loader_maze_from_file;
use crate::context::SOLVER_CONTEXT;
use crate::solver::draw::looper_realtime;
use crate::solver::solve::get_conf;
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;

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

    looper_realtime(maze, margin, padding, cell_width, cell_height).await
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
