mod command;
mod core;
mod ui;

use crate::command::ArgCommand::{Create, Solve, View};
use crate::command::{get_contexts, AmazeingArgs, CreateArgs, SolveArgs, ViewArgs};
use crate::ui::display_loop::{
    generate_loop, generate_simulation_loop, solve_loop, solve_simulation_loop, update_loop, view_loop,
};
use crate::ui::helper::generate_maze_tiles;
use crate::ui::shape::MazeScene;
use clap::Parser;
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::Conf;

#[macroquad::main(window_config())]
async fn main() {
    let args = AmazeingArgs::parse();
    let (amz_ctx, draw_context, color_context) = get_contexts(args.clone());

    match args.command.clone() {
        Create(CreateArgs { verbose: false, .. }) => {
            let ctx = amz_ctx.0.unwrap();
            let mut maze = generate_maze_tiles(ctx.rows, ctx.cols, &draw_context);
            let mut scene = MazeScene::new(&maze, draw_context.unit_shape, draw_context.zoom, &color_context);
            set_screen_size(scene.dimension);
            generate_loop(&mut scene, &mut maze, &ctx, &draw_context, &color_context).await
        }
        Create(CreateArgs { verbose: true, .. }) => {
            let ctx = amz_ctx.0.unwrap();
            let mut maze = generate_maze_tiles(ctx.rows, ctx.cols, &draw_context);
            let mut scene = MazeScene::new(&maze, draw_context.unit_shape, draw_context.zoom, &color_context);
            set_screen_size(scene.dimension);
            generate_simulation_loop(&mut scene, &mut maze, &ctx, &draw_context, &color_context).await
        }
        View(ViewArgs { update: false, .. }) => {
            let ctx = amz_ctx.1.unwrap();
            let scene = MazeScene::new(&ctx.maze, draw_context.unit_shape, draw_context.zoom, &color_context);
            set_screen_size(scene.dimension);
            view_loop(scene, &ctx, &draw_context, &color_context).await
        }
        View(ViewArgs { update: true, .. }) => {
            let ctx = amz_ctx.1.unwrap();
            let mut scene = MazeScene::new(&ctx.maze, draw_context.unit_shape, draw_context.zoom, &color_context);
            set_screen_size(scene.dimension);
            update_loop(&mut scene, &ctx, &draw_context, &color_context).await
        }
        Solve(SolveArgs { verbose: false, .. }) => {
            let ctx = amz_ctx.2.unwrap();
            let mut scene = MazeScene::new(&ctx.maze, draw_context.unit_shape, draw_context.zoom, &color_context);
            set_screen_size(scene.dimension);
            solve_loop(&mut scene, &ctx, &draw_context, &color_context).await
        }
        Solve(SolveArgs { verbose: true, .. }) => {
            let ctx = amz_ctx.2.unwrap();
            let mut scene = MazeScene::new(&ctx.maze, draw_context.unit_shape, draw_context.zoom, &color_context);
            set_screen_size(scene.dimension);
            solve_simulation_loop(&mut scene, &ctx, &draw_context, &color_context).await
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
