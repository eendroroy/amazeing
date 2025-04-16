use clap::{Command, CommandFactory};
use clap_complete::Shell::{Bash, Fish, PowerShell, Zsh};

#[path = "src/command/args.rs"]
mod args;

fn main() {
    let package = env!("CARGO_PKG_NAME");

    completions(package);
    man(package, "../contrib/man", args::AmazeingArgs::command());
    man(&format!("{package}-create"), "../contrib/man", args::CreateArgs::command().bin_name(&format!("{package} create")));
    man(&format!("{package}-view"), "../contrib/man", args::ViewArgs::command().bin_name(&format!("{package} view")));
    man(&format!("{package}-solve"), "../contrib/man", args::SolveArgs::command().bin_name(&format!("{package} solve")));
}

fn completions(bin_name: &str) {
    let out_dir = "../contrib/completions";

    let command = &mut args::AmazeingArgs::command();

    clap_complete::generate_to(Bash, command, bin_name, out_dir).unwrap();
    clap_complete::generate_to(Fish, command, bin_name, out_dir).unwrap();
    clap_complete::generate_to(PowerShell, command, bin_name, out_dir).unwrap();
    clap_complete::generate_to(Zsh, command, bin_name, out_dir).unwrap();
}

fn man(bin_name: &str, out_dir: &str, command: Command) {
    let man = clap_mangen::Man::new(command);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer).unwrap();

    std::fs::write(format!("{out_dir}/{bin_name}.1"), buffer).unwrap();
}
