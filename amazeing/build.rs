use clap::CommandFactory;
use clap_complete::Shell::{Bash, Elvish, Fish, PowerShell, Zsh};

#[path = "src/command/args.rs"]
mod args;

fn main() {
    completions();
    man();
}

fn completions() {
    let bin_name = env!("CARGO_PKG_NAME");
    let out_dir = "../contrib/completions";

    let command = &mut args::AmazeingArgs::command();

    clap_complete::generate_to(Bash, command, bin_name, out_dir).unwrap();
    clap_complete::generate_to(Elvish, command, bin_name, out_dir).unwrap();
    clap_complete::generate_to(Fish, command, bin_name, out_dir).unwrap();
    clap_complete::generate_to(PowerShell, command, bin_name, out_dir).unwrap();
    clap_complete::generate_to(Zsh, command, bin_name, out_dir).unwrap();
}

fn man() {
    let bin_name = env!("CARGO_PKG_NAME");
    let out_dir = "../contrib/man";

    let command = args::AmazeingArgs::command();

    let man = clap_mangen::Man::new(command);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer).unwrap();

    std::fs::write(format!("{out_dir}/{bin_name}.1"), buffer).unwrap();
}
