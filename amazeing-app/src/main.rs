use colored::Colorize;
use std::env;

pub(crate) mod solver;

pub static mut PATH: String = String::new();

fn header(text: &str) -> String {
    format!("{}", text.truecolor(162, 190, 140).bold())
}

fn command(text: &str) -> String {
    format!("{}", text.truecolor(143, 188, 187).bold())
}

fn value(text: &str) -> String {
    format!("{}", text.truecolor(135, 192, 208))
}

fn description(text: &str) -> String {
    format!("{}", text.truecolor(216, 222, 233))
}

fn help() {
    println!(
        "{} {} {}",
        header("Usage:"),
        command("amezing"),
        value("[options]")
    );
    println!();
    println!("{}", header("Options:"));
    println!(
        "{} {}",
        command("    -h, --help"),
        description("          Print the help menu")
    );
    println!(
        "{} {}",
        command("        --ui-cli"),
        description("        Run the simulation in cli mode instead of gui")
    );
    println!(
        "{} {}",
        command("        --bfs"),
        description("           Run the simulation for BFS")
    );
    println!(
        "{} {}",
        command("        --dfs"),
        description("           Run the simulation for DFS")
    );
    println!(
        "{} {} {}",
        command("        --path"),
        value("<str>"),
        description("    Path to the maze file"),
    );
    std::process::exit(0);
}

fn main() {
    let mut args = env::args().skip(1);

    let mut ui_cli = false;
    let mut simulation_name = "";

    while let Some(arg) = args.next() {
        match &arg[..] {
            "-h" | "--help" => help(),
            "--ui-cli" => ui_cli = true,
            "--bfs" => simulation_name = "bfs",
            "--dfs" => simulation_name = "dfs",
            "--path" => unsafe { PATH = args.next().unwrap() },
            _ => {
                if arg.starts_with('-') {
                    println!("Unknown argument {}", arg);
                } else {
                    println!("Unknown positional argument {}", arg);
                }
            }
        }
    }

    match (ui_cli, simulation_name) {
        (true, "bfs") => solver::cli::bfs::visualize(),
        (true, "dfs") => solver::cli::dfs::visualize(),
        (false, "bfs") => solver::gui::bfs::main(),
        (false, "dfs") => solver::gui::dfs::main(),
        _ => help(),
    }
}
