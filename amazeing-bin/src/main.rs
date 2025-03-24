use crate::command::ArgMode::{Create, Solve, View};
use crate::command::{get_contexts, AmazeingArgs};
use crate::display_loop::{
    generate_loop, generate_simulation_loop, solve_loop, solve_simulation_loop, update_loop,
    view_loop,
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

    let (screen_width, screen_height) = draw_context.screen_size();

    set_window_size(screen_width, screen_height + 30);

    match args.mode.clone() {
        Create { simulate: true, .. } => {
            generate_simulation_loop(amz_ctx.0.unwrap(), &draw_context, &color_context).await
        }
        Create { simulate: false, .. } => {
            generate_loop(amz_ctx.0.unwrap(), &draw_context, &color_context).await
        },
        View { update: false, .. } => {
            view_loop(amz_ctx.1.unwrap(), &draw_context, &color_context).await
        }
        View { update: true, .. } => {
            update_loop(amz_ctx.1.unwrap(), &draw_context, &color_context).await
        }
        Solve { simulate: true, .. } => {
            solve_simulation_loop(amz_ctx.2.unwrap(), &draw_context, &color_context).await
        }
        Solve { simulate: false, .. } => {
            solve_loop(amz_ctx.2.unwrap(), &draw_context, &color_context).await
        },
    }
}
