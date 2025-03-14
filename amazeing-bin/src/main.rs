use crate::command::{update_context, AmazeingArgs, ArgMode};
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
        ArgMode::Create { view: visualize, simulate, .. } => command::generate(simulate, visualize),
        ArgMode::View { update: modify, .. } => command::visualize(modify),
        ArgMode::Solve { simulate, .. } => command::solve(simulate),
    }
}
