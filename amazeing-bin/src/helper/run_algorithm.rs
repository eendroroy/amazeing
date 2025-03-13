use crate::command::{ArgGenProcedure, ArgSolveProcedure};
use amazeing::maze::matrix::Maze;
use amazeing::solver::matrix::{a_star, bfs, dfs, dijkstra};
use amazeing::{generator, DNode, HeuFn};

pub(crate) fn run_algorithm(
    maze: &Maze,
    from: DNode,
    to: DNode,
    procedure: ArgSolveProcedure,
    heuristic: Option<HeuFn>,
    tracer: &mut Option<Vec<Vec<DNode>>>,
) -> Vec<DNode> {
    println!("{:?} ==> {:?}", from, to);
    match procedure {
        ArgSolveProcedure::Bfs => bfs(maze, from, to, tracer),
        ArgSolveProcedure::Dfs => dfs(maze, from, to, tracer),
        ArgSolveProcedure::Dijkstra => dijkstra(maze, from, to, tracer),
        ArgSolveProcedure::AStar => a_star(maze, from, to, heuristic.unwrap(), tracer),
    }
}

pub(crate) fn generate_maze(
    maze: &mut Maze,
    from: DNode,
    procedure: &ArgGenProcedure,
    tracer: &mut Option<Vec<Vec<DNode>>>,
) {
    match procedure {
        ArgGenProcedure::Random => generator::matrix::random(maze, from, tracer),
        ArgGenProcedure::Dfs => generator::matrix::dfs(maze, from, tracer),
    }
}
