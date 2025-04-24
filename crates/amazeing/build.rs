#[path = "src/command/args.rs"]
mod args;

use crate::args::AmazeingArgs;
use clap::CommandFactory;
use clap_complete::Shell::{Bash, Fish, Zsh};

fn main() {
    let package = env!("CARGO_PKG_NAME");

    completions(package, "../../contrib/completions");
}

fn completions(bin_name: &str, out_dir: &str) {
    let command = &mut AmazeingArgs::command();

    clap_complete::generate_to(Bash, command, bin_name, out_dir).unwrap();
    clap_complete::generate_to(Fish, command, bin_name, out_dir).unwrap();
    clap_complete::generate_to(Zsh, command, bin_name, out_dir).unwrap();
}
