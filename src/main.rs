mod command;
mod core;
mod ui;

use crate::command::ArgCommand::{Create, Solve, View};
use crate::command::{AmazeingArgs, CreateArgs, SolveArgs, ViewArgs, get_contexts};
use crate::ui::component::scene::MazeScene;
use crate::ui::display_loop::{
    generate_loop, generate_simulation_loop, solve_loop, solve_simulation_loop, update_loop, view_loop,
};
use crate::ui::helper::generate_maze_tiles;
use clap::Parser;
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::Conf;

#[macroquad::main(window_config())]
async fn main() {
    let args = AmazeingArgs::parse();
    let (amazeing_context, draw_context, colors) = get_contexts(args.clone());

    match args.command.clone() {
        Create(CreateArgs { verbose: false, .. }) => {
            let mut maze = generate_maze_tiles(amazeing_context.rows, amazeing_context.cols, &draw_context);
            let mut scene = MazeScene::new(&maze, draw_context.unit_shape, draw_context.zoom, &colors);
            set_screen_size(scene.dimension);
            generate_loop(&mut scene, &mut maze, &amazeing_context, &draw_context, &colors).await
        }
        Create(CreateArgs { verbose: true, .. }) => {
            let mut maze = generate_maze_tiles(amazeing_context.rows, amazeing_context.cols, &draw_context);
            let mut scene = MazeScene::new(&maze, draw_context.unit_shape, draw_context.zoom, &colors);
            set_screen_size(scene.dimension);
            generate_simulation_loop(&mut scene, &mut maze, &amazeing_context, &draw_context, &colors).await
        }
        View(ViewArgs { update: false, .. }) => {
            let scene = MazeScene::new(&amazeing_context.maze, draw_context.unit_shape, draw_context.zoom, &colors);
            set_screen_size(scene.dimension);
            view_loop(scene, &amazeing_context, &draw_context, &colors).await
        }
        View(ViewArgs { update: true, .. }) => {
            let mut scene = MazeScene::new(&amazeing_context.maze, draw_context.unit_shape, draw_context.zoom, &colors);
            set_screen_size(scene.dimension);
            update_loop(&mut scene, &amazeing_context, &draw_context, &colors).await
        }
        Solve(SolveArgs { verbose: false, .. }) => {
            let mut scene = MazeScene::new(&amazeing_context.maze, draw_context.unit_shape, draw_context.zoom, &colors);
            set_screen_size(scene.dimension);
            solve_loop(&mut scene, &amazeing_context, &draw_context, &colors).await
        }
        Solve(SolveArgs { verbose: true, .. }) => {
            let mut scene = MazeScene::new(&amazeing_context.maze, draw_context.unit_shape, draw_context.zoom, &colors);
            set_screen_size(scene.dimension);
            solve_simulation_loop(&mut scene, &amazeing_context, &draw_context, &colors).await
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
