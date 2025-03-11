use crate::command::{get_heuristic_fn, Proc};
use crate::context::CONTEXT;
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
        CONTEXT.read().unwrap().proc,
        get_heuristic_fn(CONTEXT.read().unwrap().heuristic),
    );
    println!("{:?} ==> {:?}", from, to);
    match algorithm {
        Proc::Bfs => bfs(maze, from, to, tracer),
        Proc::Dfs => dfs(maze, from, to, tracer),
        Proc::Dijkstra => dijkstra(maze, from, to, tracer),
        Proc::AStar => a_star(maze, from, to, heu, tracer),
        _ => panic!("No algorithm provided"),
    }
}
