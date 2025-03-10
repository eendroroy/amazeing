use crate::context::SOLVER_CONTEXT;
use crate::helper::get_heu;
use amazeing::maze::matrix::Maze;
use amazeing::solver::matrix::{a_star, bfs, dfs, dijkstra};
use amazeing::DNode;

pub(crate) fn run_algorithm(maze: &Maze, from: DNode, to: DNode) -> Vec<DNode> {
    let (algorithm, heu) = (
        &*SOLVER_CONTEXT.read().unwrap().algorithm,
        get_heu(&*SOLVER_CONTEXT.read().unwrap().heu),
    );
    match algorithm {
        "bfs" => bfs(maze, from, to, &mut None),
        "dfs" => dfs(maze, from, to, &mut None),
        "dijkstra" => dijkstra(maze, from, to, &mut None),
        "a-star" => a_star(maze, from, to, heu, &mut None),
        name => panic!("Unknown algorithm name {}", name),
    }
}
