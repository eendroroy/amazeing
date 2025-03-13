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
        ArgMode::Generate { visualize, .. } => command::generate(visualize),
        ArgMode::Visualize { modify, .. } => command::visualize(modify),
        ArgMode::Solve { simulate, .. } => command::solve(simulate),
    }
}
