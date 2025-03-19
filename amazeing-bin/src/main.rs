use crate::command::ArgMode::{Create, Solve, View};
use crate::command::{generate, solve, update_context, visualize, AmazeingArgs};
use clap::Parser;

mod command;
mod context;
mod display;
mod helper;
mod ui;

fn main() {
    let args = AmazeingArgs::parse();
    update_context(args.clone());

    match args.mode.clone() {
        Create { view: visualize, simulate, .. } => generate(simulate, visualize),
        View { update: modify, .. } => visualize(modify),
        Solve { simulate, .. } => solve(simulate),
    }
}
