mod command;
mod core;
mod ui;

use crate::command::ArgCommand::{Create, Solve, View};
use crate::command::{AmazeingArgs, CreateArgs, SolveArgs, ViewArgs, get_contexts};
use crate::ui::display_loop::{
    generate_loop, generate_simulation_loop, solve_loop, solve_simulation_loop, update_loop, view_loop,
};
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
            set_screen_size(draw_context.screen_size(ctx.rows, ctx.cols));
            generate_loop(&ctx, &draw_context, &color_context).await
        }
        Create(CreateArgs { verbose: true, .. }) => {
            let ctx = amz_ctx.0.unwrap();
            set_screen_size(draw_context.screen_size(ctx.rows, ctx.cols));
            generate_simulation_loop(&ctx, &draw_context, &color_context).await
        }
        View(ViewArgs { update: false, .. }) => {
            let ctx = amz_ctx.1.unwrap();
            set_screen_size(draw_context.screen_size(ctx.maze.rows(), ctx.maze.cols()));
            view_loop(&ctx, &draw_context, &color_context).await
        }
        View(ViewArgs { update: true, .. }) => {
            let ctx = amz_ctx.1.unwrap();
            set_screen_size(draw_context.screen_size(ctx.maze.rows(), ctx.maze.cols()));
            update_loop(&ctx, &draw_context, &color_context).await
        }
        Solve(SolveArgs { verbose: false, .. }) => {
            let ctx = amz_ctx.2.unwrap();
            set_screen_size(draw_context.screen_size(ctx.maze.rows(), ctx.maze.cols()));
            solve_loop(&ctx, &draw_context, &color_context).await
        }
        Solve(SolveArgs { verbose: true, .. }) => {
            let ctx = amz_ctx.2.unwrap();
            set_screen_size(draw_context.screen_size(ctx.maze.rows(), ctx.maze.cols()));
            solve_simulation_loop(&ctx, &draw_context, &color_context).await
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
