use crate::context::{Colors, SolverContext};
use crate::matrix::loader::{loader_maze_data_from_file, parse_node};
use amazeing::solver::matrix::{
    a_star, bfs, chebyshev_heuristic, dfs, dijkstra, dijkstra_heuristic, euclidean_heuristic,
    manhattan_heuristic, octile_heuristic,
};
use colored::Colorize;
use macroquad::prelude::Conf;
use std::path::Path;
use std::sync::{LazyLock, RwLock};
use std::{env, fs};

mod cli;
mod context;
mod gui;
mod matrix;

pub static COLORS: LazyLock<Colors> = LazyLock::new(|| Colors::new());
pub static SOLVER_CONTEXT: LazyLock<RwLock<SolverContext>> = LazyLock::new(|| {
    RwLock::new(SolverContext::new())
});

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
        "{} {} {} {} {} {} {} {} {} {} {}{} {}",
        header("Usage:"),
        command("amazeing-solve"),
        command("--path"),
        value("path/to/maze.txt"),
        command("--from"),
        value("usize,usize"),
        command("--to"),
        value("usize,usize"),
        command("<--bfs|--dfs|--dijkstra|--a-star>"),
        command("[--hue"),
        value("heuristic_name"),
        command("]"),
        command("[--ui-cli]"),
    );
    println!();
    println!("{}", header("Options:"));
    println!(
        "{} {}",
        command("    -h, --help"),
        description("                Print the help menu")
    );
    println!(
        "{} {} {}",
        command("        --path"),
        value("    str"),
        description("        Path to the maze file"),
    );
    println!(
        "{} {} {}",
        command("        --from"),
        value("    usize,usize"),
        description("Start point"),
    );
    println!(
        "{} {} {}",
        command("        --to"),
        value("      usize,usize"),
        description("End point"),
    );
    println!(
        "{} {}",
        command("        --ui-cli"),
        description("              Run the simulation in cli mode instead of gui")
    );
    println!(
        "{} {}",
        command("        --bfs"),
        description("                 Run the simulation for BFS")
    );
    println!(
        "{} {}",
        command("        --dfs"),
        description("                 Run the simulation for DFS")
    );
    println!(
        "{} {}",
        command("        --dijkstra"),
        description("            Run the simulation for Dijkstra")
    );
    println!(
        "{} {}",
        command("        --a-star"),
        description("              Run the simulation for A*")
    );
    println!(
        "{} {} {} {} {} {}",
        command("        --heu"),
        value("     str"),
        description("        Heuristic function to use with A*"),
        description("\n                               Choose from:"),
        description("manhattan, euclidean, chebyshev, octile, dijkstra"),
        description("\n                               Default dijkstra if none provided"),
    );
    println!(
        "{} {} {}",
        command("        --fps"),
        value("     u8"),
        description("         Gui FPS"),
    );
    std::process::exit(0);
}

pub fn get_conf() -> Conf {
    Conf {
        window_title: SOLVER_CONTEXT.read().unwrap().title.clone(),
        ..Default::default()
    }
}

fn main() {
    let mut args = env::args().skip(1);

    let mut ui_cli = false;
    let mut simulation_name = "";
    let mut heu = String::from("");
    let mut path = String::from("");
    let mut from = String::from("");
    let mut to = String::from("");
    let mut fps = String::from("");

    while let Some(arg) = args.next() {
        match &arg[..] {
            "-h" | "--help" => help(),
            "--ui-cli" => ui_cli = true,
            "--bfs" => simulation_name = "bfs",
            "--dfs" => simulation_name = "dfs",
            "--dijkstra" => simulation_name = "dijkstra",
            "--a-star" => simulation_name = "a-star",
            "--heu" => heu = args.next().unwrap(),
            "--path" => path = args.next().unwrap(),
            "--from" => from = args.next().unwrap(),
            "--to" => to = args.next().unwrap(),
            "--fps" => fps = args.next().unwrap(),
            _ => {
                if arg.starts_with('-') {
                    println!("Unknown argument {}", arg);
                } else {
                    println!("Unknown positional argument {}", arg);
                }
            }
        }
    }

    if !fs::exists(Path::new(&path)).unwrap() {
        panic!("Maze file {} does not exists", path)
    } else {
            SOLVER_CONTEXT.write().unwrap().maze = loader_maze_data_from_file(&*path);
    }

    if fps != String::from("") {
        SOLVER_CONTEXT.write().unwrap().fps = u8::from_str_radix(&fps, 10).unwrap();
    }

    let maze = SOLVER_CONTEXT.read().unwrap().maze.clone();

    let mut tracer: Option<Vec<Vec<(usize, usize)>>> = Some(vec![]);

    let maze_path = match simulation_name {
        "bfs" => {
            SOLVER_CONTEXT.write().unwrap().title = String::from("Maze Solver (BFS)");
            bfs(&maze, parse_node(&from), parse_node(&to), &mut tracer)
        },
        "dfs" => {
            SOLVER_CONTEXT.write().unwrap().title = String::from("Maze Solver (DFS)");
            dfs(&maze, parse_node(&from), parse_node(&to), &mut tracer)
        },
        "dijkstra" => {
            SOLVER_CONTEXT.write().unwrap().title = String::from("Maze Solver (Dijkstra)");
            dijkstra(&maze, parse_node(&from), parse_node(&to), &mut tracer)
        },
        "a-star" => {
            SOLVER_CONTEXT.write().unwrap().title = String::from("Maze Solver (A*)");
            let heuristic = match &*heu {
                "manhattan" => manhattan_heuristic,
                "euclidean" => euclidean_heuristic,
                "chebyshev" => chebyshev_heuristic,
                "octile" => octile_heuristic,
                "dijkstra" => dijkstra_heuristic,
                _ => dijkstra_heuristic,
            };
            a_star(
                &maze,
                parse_node(&from),
                parse_node(&to),
                heuristic,
                &mut tracer,
            )
        }
        _ => panic!("Unknown simulation name {}", simulation_name),
    };

    SOLVER_CONTEXT.write().unwrap().tracer = tracer.unwrap();

    match ui_cli {
        true => cli::visualize::visualize(&maze, maze_path),
        false => gui::simulation::main(),
    }
}
