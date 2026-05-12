mod bound;
mod color_gradient;
mod display_helpers;
mod file;
mod mouse;
mod run_algorithm;
mod time;

pub(crate) use bound::is_point_in_triangle;
pub(crate) use color_gradient::gradient;
pub(crate) use display_helpers::{save_maze, take_a_snap};
pub(crate) use file::{dump_maze_to_file, load_maze_from_file};
pub(crate) use mouse::handle_mouse_click;
pub(crate) use run_algorithm::{generate_maze, generate_maze_stream, solve_maze, solve_maze_stream};
pub(crate) use time::current_millis;
