use crate::cli::{update_context, ArgMode};
use clap::Parser;

mod cli;
mod context;
mod display;
mod helper;
mod mode;

fn main() {
    let args = cli::AmazeingArgs::parse();
    update_context(args.clone());

    match args.mode.clone() {
        ArgMode::Generate { visualize, .. } => mode::generate(visualize),
        ArgMode::Visualize { .. } => mode::visualize(),
        ArgMode::Modify { .. } => mode::modify(),
        ArgMode::Solve { simulate: false, .. } => mode::solve(),
        ArgMode::Solve { simulate: true, .. } => mode::solve_simulate(),
    }
}
