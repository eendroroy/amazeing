use crate::command::ArgMode::{Create, Solve, View};
use crate::command::{update_context, AmazeingArgs};
use crate::context::DRAW_CTX;
use crate::ui::generate_loop::generate_loop;
use crate::ui::generate_simulate::generate_simulation_loop;
use crate::ui::update::update_loop;
use crate::ui::view::view_loop;
use clap::Parser;
use macroquad::miniquad::window::set_window_size;
use crate::ui::solve::solve_loop;
use crate::ui::solve_simulate::solve_simulation_loop;

mod command;
mod context;
mod helper;
mod ui;

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
