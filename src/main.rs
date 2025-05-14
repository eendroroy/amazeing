mod command;
mod core;
mod ui;

use crate::command::ArgCommand::{Create, Solve, View};
use crate::command::{AmazeingArgs, CreateArgs, SolveArgs, ViewArgs, get_contexts};
use crate::core::tiled::{Maze, VOID};
use crate::ui::component::scene::MazeScene;
use crate::ui::display_loop::{
    generate_loop, generate_simulation_loop, solve_loop, solve_simulation_loop, update_loop, view_loop,
};
use clap::Parser;
use macroquad::color::YELLOW;
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::Conf;

#[macroquad::main(window_config())]
async fn main() {
    let args = AmazeingArgs::parse();
    let (amazeing_context, colors) = get_contexts(args.clone());

    match args.command.clone() {
        Create(CreateArgs {
            verbose: false,
            maze_shape,
            unit_shape,
            ..
        }) => {
            let maze =
                Maze::new(maze_shape.shape(), unit_shape.shape(), amazeing_context.rows, amazeing_context.cols, VOID);
            let mut scene = MazeScene::new(&maze, args.zoom, args.fps, &colors);
            scene.set_bound(YELLOW);
            set_screen_size(scene.dimension);
            generate_loop(&mut scene, &amazeing_context, &colors).await
        }
        Create(CreateArgs {
            verbose: true,
            maze_shape,
            unit_shape,
            ..
        }) => {
            let maze =
                Maze::new(maze_shape.shape(), unit_shape.shape(), amazeing_context.rows, amazeing_context.cols, VOID);
            let mut scene = MazeScene::new(&maze, args.zoom, args.fps, &colors);
            scene.set_bound(YELLOW);
            set_screen_size(scene.dimension);
            generate_simulation_loop(&mut scene, &amazeing_context, &colors).await
        }
        View(ViewArgs { update: false, .. }) => {
            let scene = MazeScene::new(&amazeing_context.maze.clone().unwrap(), args.zoom, args.fps, &colors);
            set_screen_size(scene.dimension);
            view_loop(scene, &colors).await
        }
        View(ViewArgs { update: true, .. }) => {
            let mut scene = MazeScene::new(&amazeing_context.maze.clone().unwrap(), args.zoom, args.fps, &colors);
            set_screen_size(scene.dimension);
            update_loop(&mut scene, &amazeing_context, &colors).await
        }
        Solve(SolveArgs { verbose: false, .. }) => {
            let mut scene = MazeScene::new(&amazeing_context.maze.clone().unwrap(), args.zoom, args.fps, &colors);
            set_screen_size(scene.dimension);
            solve_loop(&mut scene, &amazeing_context, &colors).await
        }
        Solve(SolveArgs { verbose: true, .. }) => {
            let mut scene = MazeScene::new(&amazeing_context.maze.clone().unwrap(), args.zoom, args.fps, &colors);
            set_screen_size(scene.dimension);
            solve_simulation_loop(&mut scene, &amazeing_context, &colors).await
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
