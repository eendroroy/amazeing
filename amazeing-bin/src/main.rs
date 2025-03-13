use crate::cli::{update_context, ArgMode};
use clap::Parser;

mod cli;
mod command;
mod context;
mod display;
mod helper;
mod ui;

fn main() {
    let args = cli::AmazeingArgs::parse();
    update_context(args.clone());

    match args.mode.clone() {
        ArgMode::Generate { visualize, .. } => command::generate(visualize),
        ArgMode::Visualize { modify, .. } => command::visualize(modify),
        ArgMode::Solve { simulate, .. } => command::solve(simulate),
    }
}
