mod dispatch;
mod file;
mod input;
mod math;

pub(crate) use dispatch::{generate_maze, generate_maze_stream, solve_maze, solve_maze_stream};
pub(crate) use file::{dump_maze_to_file, load_maze_from_file};
pub(crate) use input::{handle_mouse_click, save_maze, take_a_snap};
pub(crate) use math::{current_millis, gradient, is_point_in_triangle};
