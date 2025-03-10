use crate::context::CONTEXT;
use crate::helper::get_heu;
use amazeing::maze::matrix::Maze;
use amazeing::solver::matrix::{a_star, bfs, dfs, dijkstra};
use amazeing::DNode;

pub(crate) fn run_algorithm(
    maze: &Maze,
    from: DNode,
    to: DNode,
    tracer: &mut Option<Vec<Vec<DNode>>>,
) -> Vec<DNode> {
    let (algorithm, heu) = (
        &*CONTEXT.read().unwrap().proc,
        get_heu(&*CONTEXT.read().unwrap().heu),
    );
    println!("{} -- {:?} ==> {:?}", algorithm, from, to);
    match algorithm {
        "bfs" => bfs(maze, from, to, tracer),
        "dfs" => dfs(maze, from, to, tracer),
        "dijkstra" => dijkstra(maze, from, to, tracer),
        "a-star" => a_star(maze, from, to, heu, tracer),
        name => panic!("Unknown algorithm name {}", name),
    }
}
