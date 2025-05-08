mod command;
mod core;
mod ui;

use crate::command::ArgCommand::{Create, Solve, View};
use crate::command::{AmazeingArgs, CreateArgs, SolveArgs, ViewArgs, get_contexts};
use crate::ui::display_loop::{
    generate_loop, generate_simulation_loop, solve_loop, solve_simulation_loop, update_loop, view_loop,
};
use crate::ui::helper::{convert_to_maze_shape, generate_maze_tiles};
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
            let (dimension, mut shapes) = convert_to_maze_shape(&maze, &draw_context, &color_context);
            set_screen_size(dimension);
            generate_loop(&mut shapes, &mut maze, &ctx, &draw_context, &color_context).await
        }
        Create(CreateArgs { verbose: true, .. }) => {
            let ctx = amz_ctx.0.unwrap();
            let mut maze = generate_maze_tiles(ctx.rows, ctx.cols, &draw_context);
            let (dimension, mut shapes) = convert_to_maze_shape(&maze, &draw_context, &color_context);
            set_screen_size(dimension);
            generate_simulation_loop(&mut shapes, &mut maze, &ctx, &draw_context, &color_context).await
        }
        View(ViewArgs { update: false, .. }) => {
            let ctx = amz_ctx.1.unwrap();
            let (dimension, shapes) = convert_to_maze_shape(&ctx.maze, &draw_context, &color_context);
            set_screen_size(dimension);
            view_loop(shapes, &ctx, &draw_context, &color_context).await
        }
        View(ViewArgs { update: true, .. }) => {
            let ctx = amz_ctx.1.unwrap();
            let (dimension, mut shapes) = convert_to_maze_shape(&ctx.maze, &draw_context, &color_context);
            set_screen_size(dimension);
            update_loop(&mut shapes, &ctx, &draw_context, &color_context).await
        }
        Solve(SolveArgs { verbose: false, .. }) => {
            let ctx = amz_ctx.2.unwrap();
            let (dimension, mut shapes) = convert_to_maze_shape(&ctx.maze, &draw_context, &color_context);
            set_screen_size(dimension);
            solve_loop(&mut shapes, &ctx, &draw_context, &color_context).await
        }
        Solve(SolveArgs { verbose: true, .. }) => {
            let ctx = amz_ctx.2.unwrap();
            let (dimension, mut shapes) = convert_to_maze_shape(&ctx.maze, &draw_context, &color_context);
            set_screen_size(dimension);
            solve_simulation_loop(&mut shapes, &ctx, &draw_context, &color_context).await
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
