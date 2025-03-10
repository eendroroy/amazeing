use crate::command::loader::loader_maze_from_file;
use crate::command::parse_node::parse_node;
use crate::context::SOLVER_CONTEXT;
use crate::solver;
use amazeing::maze::matrix::{
    chebyshev_heuristic, dijkstra_heuristic, euclidean_heuristic, manhattan_heuristic,
    octile_heuristic,
};
use amazeing::solver::matrix::{a_star, bfs, dfs, dijkstra};
use amazeing::DNode;
use macroquad::prelude::Conf;

pub fn get_conf() -> Conf {
    Conf {
        window_title: SOLVER_CONTEXT.read().unwrap().title.clone(),
        ..Default::default()
    }
}

pub(crate) fn get_heu(heu: &str) -> fn(DNode, DNode) -> u32 {
    match heu {
        "manhattan" => manhattan_heuristic,
        "euclidean" => euclidean_heuristic,
        "chebyshev" => chebyshev_heuristic,
        "octile" => octile_heuristic,
        "dijkstra" => dijkstra_heuristic,
        _ => dijkstra_heuristic,
    }
}

pub(crate) fn solve(
    algorithm: String,
    heu: String,
    maze_file_path: String,
    from: String,
    to: String,
    fps: String,
    display_size: String,
) {
    SOLVER_CONTEXT.write().unwrap().title = String::from(format!("Maze Solver ({})", algorithm));
    SOLVER_CONTEXT.write().unwrap().display_size = display_size;

    if fps != String::from("") {
        SOLVER_CONTEXT.write().unwrap().fps = u8::from_str_radix(&fps, 10).unwrap();
    }

    let maze = loader_maze_from_file(&*maze_file_path);
    SOLVER_CONTEXT.write().unwrap().maze = maze.clone();

    let mut tracer: Option<Vec<Vec<DNode>>> = Some(vec![]);
    let (source, destination) = (parse_node(&from), parse_node(&to));

    SOLVER_CONTEXT.write().unwrap().source = source;
    SOLVER_CONTEXT.write().unwrap().destination = destination;

    match &*algorithm {
        "bfs" => bfs(&maze, source, destination, &mut tracer),
        "dfs" => dfs(&maze, source, destination, &mut tracer),
        "dijkstra" => dijkstra(&maze, source, destination, &mut tracer),
        "a-star" => a_star(&maze, source, destination, get_heu(&*heu), &mut tracer),
        _ => panic!("Unknown algorithm name {}", algorithm),
    };

    SOLVER_CONTEXT.write().unwrap().tracer = tracer.unwrap();

    solver::simulation::main();
}
