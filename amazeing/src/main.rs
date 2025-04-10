use crate::command::ArgCommand::{Create, Solve, View};
use crate::command::{AmazeingArgs, get_contexts};
use crate::display_loop::{
    generate_loop, generate_simulation_loop, solve_loop, solve_simulation_loop, update_loop, view_loop,
};
use clap::Parser;
use macroquad::miniquad::window::set_window_size;

mod command;
mod context;
mod display_loop;
mod helper;

#[macroquad::main("Amazeing")]
async fn main() {
    let args = AmazeingArgs::parse();
    let (amz_ctx, draw_context, color_context) = get_contexts(args.clone());

    match args.command.clone() {
        Create { verbose: true, .. } => {
            let ctx = amz_ctx.0.unwrap();
            set_screen_size(draw_context.screen_size(ctx.rows, ctx.cols));
            generate_simulation_loop(&ctx, &draw_context, &color_context).await
        }
        Create { verbose: false, .. } => {
            let ctx = amz_ctx.0.unwrap();
            set_screen_size(draw_context.screen_size(ctx.rows, ctx.cols));
            generate_loop(&ctx, &draw_context, &color_context).await
        }
        View { update: false, .. } => {
            let ctx = amz_ctx.1.unwrap();
            set_screen_size(draw_context.screen_size(ctx.maze.rows(), ctx.maze.cols()));
            view_loop(&ctx, &draw_context, &color_context).await
        }
        View { update: true, .. } => {
            let ctx = amz_ctx.1.unwrap();
            set_screen_size(draw_context.screen_size(ctx.maze.rows(), ctx.maze.cols()));
            update_loop(&ctx, &draw_context, &color_context).await
        }
        Solve { verbose: true, .. } => {
            let ctx = amz_ctx.2.unwrap();
            set_screen_size(draw_context.screen_size(ctx.maze.rows(), ctx.maze.cols()));
            solve_simulation_loop(&ctx, &draw_context, &color_context).await
        }
        Solve { verbose: false, .. } => {
            let ctx = amz_ctx.2.unwrap();
            set_screen_size(draw_context.screen_size(ctx.maze.rows(), ctx.maze.cols()));
            solve_loop(&ctx, &draw_context, &color_context).await
        }
    }
}

fn set_screen_size((width, height): (u32, u32)) {
    set_window_size(width, height + 30);
}
