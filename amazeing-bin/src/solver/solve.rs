use crate::command::parse_node::parse_node;
use crate::context::SOLVER_CONTEXT;
use crate::helper::get_heu;
use crate::helper::loader::loader_maze_from_file;
use crate::solver;
use amazeing::solver::matrix::{a_star, bfs, dfs, dijkstra};
use amazeing::DNode;

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
