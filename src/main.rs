mod command;
mod core;
mod ui;

use crate::command::ArgCommand::{Create, Solve, View};
use crate::command::{AmazeingArgs, CreateArgs, SolveArgs, ViewArgs, get_contexts};
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
    let (context, colors) = get_contexts(args.clone());

    match args.command.clone() {
        Create(CreateArgs {
            verbose: false,
            maze_shape,
            unit_shape,
            ..
        }) => {
            let mut scene = MazeScene::new_from_dimension(maze_shape.shape(), unit_shape.shape(), &context, &colors);
            scene.set_bound(YELLOW);
            set_screen_size(scene.wh);
            generate_loop(&mut scene).await
        }
        Create(CreateArgs {
            verbose: true,
            maze_shape,
            unit_shape,
            ..
        }) => {
            let mut scene = MazeScene::new_from_dimension(maze_shape.shape(), unit_shape.shape(), &context, &colors);
            scene.set_bound(YELLOW);
            set_screen_size(scene.wh);
            generate_simulation_loop(&mut scene).await
        }
        View(ViewArgs { update: false, .. }) => {
            let scene = MazeScene::new_from_maze(&context.maze.clone().unwrap(), &context, &colors);
            set_screen_size(scene.wh);
            view_loop(scene).await
        }
        View(ViewArgs { update: true, .. }) => {
            let mut scene = MazeScene::new_from_maze(&context.maze.clone().unwrap(), &context, &colors);
            set_screen_size(scene.wh);
            update_loop(&mut scene).await
        }
        Solve(SolveArgs { verbose: false, .. }) => {
            let mut scene = MazeScene::new_from_maze(&context.maze.clone().unwrap(), &context, &colors);
            set_screen_size(scene.wh);
            solve_loop(&mut scene).await
        }
        Solve(SolveArgs { verbose: true, .. }) => {
            let mut scene = MazeScene::new_from_maze(&context.maze.clone().unwrap(), &context, &colors);
            set_screen_size(scene.wh);
            solve_simulation_loop(&mut scene).await
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
