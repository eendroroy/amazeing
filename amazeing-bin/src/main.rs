use crate::command::ArgMode::{Create, Solve, View};
use crate::command::{update_context, AmazeingArgs};
use crate::context::DRAW_CTX;
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
    update_context(args.clone());

    let (screen_width, screen_height) = DRAW_CTX.read().unwrap().screen_size();

    set_window_size(screen_width, screen_height + 30);

    match args.mode.clone() {
        Create { simulate: true, .. } => generate_simulation_loop().await,
        Create { simulate: false, .. } => generate_loop().await,
        View { update: false, .. } => view_loop().await,
        View { update: true, .. } => update_loop().await,
        Solve { simulate: true, .. } => solve_simulation_loop().await,
        Solve { simulate: false, .. } => solve_loop().await,
    }
}
