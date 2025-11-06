mod command;
mod core;
mod ui;
mod utility;

use crate::command::ArgCommand::{Create, Solve, View};
use crate::command::{AmazeingArgs, AmazeingContext, ColorScheme, Colors};
use crate::ui::component::scene::MazeScene;
use crate::ui::display_loop::{
    generate_loop, generate_simulation_loop, solve_loop, solve_simulation_loop, update_loop, view_loop,
};
use crate::ui::helper::load_maze_from_file;
use clap::Parser;
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::Conf;
use std::path::PathBuf;

macro_rules! gradient_steps {
    ($r:expr, $c:expr) => {
        ((($r + $c) as f32 * 0.25).clamp(8., 64.) as usize)
    };
}

#[macroquad::main(window_config())]
async fn main() {
    let amazeing_args = AmazeingArgs::parse();

    match amazeing_args.command.clone() {
        Create(args) => {
            let context = AmazeingContext::create_context(
                (None, args.maze),
                (args.procedure, args.heuristic_function.heuristic()),
                (args.jumble_factor, args.weight_direction.direction()),
                (args.rows, args.cols),
                (amazeing_args.zoom, amazeing_args.fps, amazeing_args.show_perimeter),
            );
            let mut scene = MazeScene::new_from_dimension(
                args.unit_shape.shape(),
                &context,
                &get_colors(context.rows, context.cols, amazeing_args.colors),
            );
            set_screen_size(scene.wh);
            if args.verbose {
                generate_simulation_loop(&mut scene).await
            } else {
                generate_loop(&mut scene).await
            }
        }
        View(args) => {
            let context = AmazeingContext::view_context(
                (load_maze_from_file(args.maze.as_path()), args.maze.clone()),
                (amazeing_args.zoom, amazeing_args.fps, amazeing_args.show_perimeter),
            );
            let mut scene = MazeScene::new_from_maze(
                &context.maze.clone().unwrap(),
                &context,
                &get_colors(context.rows, context.cols, amazeing_args.colors),
            );
            set_screen_size(scene.wh);
            if args.update {
                update_loop(&mut scene).await
            } else {
                view_loop(scene).await
            }
        }
        Solve(args) => {
            let context = AmazeingContext::solve_context(
                load_maze_from_file(args.maze.as_path()),
                (args.procedure, args.heuristic_function.heuristic()),
                (amazeing_args.zoom, amazeing_args.fps, amazeing_args.show_perimeter),
            );
            let mut scene = MazeScene::new_from_maze(
                &context.maze.clone().unwrap(),
                &context,
                &get_colors(context.rows, context.cols, amazeing_args.colors),
            );
            set_screen_size(scene.wh);
            if args.verbose {
                solve_simulation_loop(&mut scene).await
            } else {
                solve_loop(&mut scene).await
            }
        }
    }
}

fn window_config() -> Conf {
    Conf {
        window_title: "Amazeing".to_string(),
        high_dpi: true,
        sample_count: 10,
        window_resizable: false,
        ..Default::default()
    }
}

fn set_screen_size((width, height): (u32, u32)) {
    set_window_size(width, height + 30);
}

fn get_colors(rows: usize, cols: usize, scheme_path: Option<PathBuf>) -> Colors {
    let gradient_steps: usize = gradient_steps![rows, cols];
    if let Some(path) = scheme_path {
        Colors::from(ColorScheme::from(path.as_path()), gradient_steps)
    } else {
        Colors::new(gradient_steps)
    }
}
