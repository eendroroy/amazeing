mod common;
mod create;
mod solve;
mod view;

use crate::cli::AmazeingArgs;
use crate::cli::ArgCommand::{Create, Solve, View};
use clap::Parser;

pub async fn run() {
    let amazeing_args = AmazeingArgs::parse();

    match amazeing_args.command.clone() {
        Create(args) => create::run(&amazeing_args, args).await,
        View(args) => view::run(&amazeing_args, args).await,
        Solve(args) => solve::run(&amazeing_args, args).await,
    }
}
