use crate::arg_parser::parse_arg;
use crate::context::{Colors, SolverContext};
use crate::matrix::loader::{loader_maze_data_from_file, parse_node};
use amazeing::solver::matrix::{
    a_star, bfs, chebyshev_heuristic, dfs, dijkstra, dijkstra_heuristic, euclidean_heuristic,
    manhattan_heuristic, octile_heuristic,
};
use macroquad::prelude::Conf;
use std::sync::{LazyLock, RwLock};

mod context;
mod gui;
mod matrix;
mod help;
mod arg_parser;

pub static COLORS: LazyLock<Colors> = LazyLock::new(|| Colors::new());
pub static SOLVER_CONTEXT: LazyLock<RwLock<SolverContext>> = LazyLock::new(|| {
    RwLock::new(SolverContext::new())
});

pub fn get_conf() -> Conf {
    Conf {
        window_title: SOLVER_CONTEXT.read().unwrap().title.clone(),
        ..Default::default()
    }
}

fn get_heu(heu: &str) -> fn((usize, usize), (usize, usize)) -> u32 {
    match heu {
        "manhattan" => manhattan_heuristic,
        "euclidean" => euclidean_heuristic,
        "chebyshev" => chebyshev_heuristic,
        "octile" => octile_heuristic,
        "dijkstra" => dijkstra_heuristic,
        _ => dijkstra_heuristic,
    }
}

fn main() {
    let (algorithm, heu, path, from, to, fps) = parse_arg();

    SOLVER_CONTEXT.write().unwrap().title = String::from(format!("Maze Solver ({})", algorithm));

    if fps != String::from("") {
        SOLVER_CONTEXT.write().unwrap().fps = u8::from_str_radix(&fps, 10).unwrap();
    }

    let maze = loader_maze_data_from_file(&*path);
    SOLVER_CONTEXT.write().unwrap().maze = maze.clone();

    let mut tracer: Option<Vec<Vec<(usize, usize)>>> = Some(vec![]);

    match &*algorithm {
        "bfs" => bfs(&maze, parse_node(&from), parse_node(&to), &mut tracer),
        "dfs" => dfs(&maze, parse_node(&from), parse_node(&to), &mut tracer),
        "dijkstra" => dijkstra(&maze, parse_node(&from), parse_node(&to), &mut tracer),
        "a-star" => a_star(&maze, parse_node(&from), parse_node(&to), get_heu(&*heu), &mut tracer),
        _ => panic!("Unknown algorithm name {}", algorithm),
    };

    SOLVER_CONTEXT.write().unwrap().tracer = tracer.unwrap();

    gui::simulation::main();
}
