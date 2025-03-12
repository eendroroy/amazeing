use crate::cli::{update_context, ArgMode};
use clap::Parser;

mod cli;
mod context;
mod display;
mod helper;
mod mode;

fn main() {
    let args = cli::Arg::parse();
    update_context(args.clone());

    match args.mode.clone() {
        ArgMode::Simulate { .. } => mode::simulate(),
        ArgMode::Realtime { .. } => mode::realtime(),
        ArgMode::Generate { .. } => mode::generate(),
        ArgMode::Visualize { .. } => mode::visualize(),
        ArgMode::Modify { .. } => mode::modify(),
    }
}
