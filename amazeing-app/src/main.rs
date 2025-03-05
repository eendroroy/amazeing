use crate::solver::matrix::loader::{loader_maze_data_from_file, parse_node};
use amazeing::solver::matrix::{
    chebyshev_heuristic, dijkstra_heuristic, euclidean_heuristic, manhattan_heuristic,
    octile_heuristic,
};
use colored::Colorize;
use macroquad::color::{Color, BEIGE, BLACK, DARKGRAY, GOLD, LIGHTGRAY, RED};
use std::path::Path;
use std::sync::Mutex;
use std::{env, fs};

pub(crate) mod solver;

pub static BG: Color = BLACK;
pub static BLOCK: Color = DARKGRAY;
pub static OPEN: Color = LIGHTGRAY;
pub static VISITING: Color = RED;
pub static PATH: Color = GOLD;
pub static TRAVERSED: Color = BEIGE;

pub static FPS: Mutex<u8> = Mutex::new(7u8);
pub static MAZE_DATA: Mutex<Vec<Vec<u32>>> = Mutex::new(vec![]);
pub static FROM: Mutex<(usize, usize)> = Mutex::new((0, 0));
pub static TO: Mutex<(usize, usize)> = Mutex::new((0, 0));
pub static HEURISTIC: Mutex<fn((usize, usize), (usize, usize)) -> u32> =
    Mutex::new(dijkstra_heuristic);

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
        command("amazeing"),
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
        loader_maze_data_from_file(&*path).iter().for_each(|row| {
            MAZE_DATA.lock().unwrap().push(row.clone());
        });
    }

    if from == String::from("") {
        panic!("Invalid start node {}", from)
    } else {
        let node = parse_node(&from);
        FROM.lock().unwrap().0 = node.0;
        FROM.lock().unwrap().1 = node.1;
    }

    if to == String::from("") {
        panic!("Invalid end node {}", to)
    } else {
        let node = parse_node(&to);
        TO.lock().unwrap().0 = node.0;
        TO.lock().unwrap().1 = node.1;
    }

    if fps != String::from("") {
        *FPS.lock().unwrap() = u8::from_str_radix(&fps, 10).unwrap();
    }

    *HEURISTIC.lock().unwrap() = match &*heu {
        "manhattan" => manhattan_heuristic,
        "euclidean" => euclidean_heuristic,
        "chebyshev" => chebyshev_heuristic,
        "octile" => octile_heuristic,
        "dijkstra" => dijkstra_heuristic,
        other => panic!("Invalid heuristic function {}", other),
    };

    match (ui_cli, simulation_name) {
        (true, "bfs") => solver::cli::bfs::visualize(),
        (true, "dfs") => solver::cli::dfs::visualize(),
        (true, "dijkstra") => solver::cli::dijkstra::visualize(),
        (true, "a-star") => solver::cli::a_star::visualize(),

        (false, "bfs") => solver::gui::bfs::main(),
        (false, "dfs") => solver::gui::dfs::main(),
        (false, "dijkstra") => solver::gui::dijkstra::main(),
        (false, "a-star") => solver::gui::a_star::main(),
        _ => help(),
    }
}
