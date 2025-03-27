use crate::command::{ArgGenProcedure, ArgSolveProcedure};
use amazeing::matrix::{Maze, Node, NodeHeuFn, Tracer};

pub(crate) fn solve_maze(
    maze: &Maze,
    from: Node,
    to: Node,
    procedure: &ArgSolveProcedure,
    heuristic: Option<NodeHeuFn>,
    tracer: &mut Option<Tracer>,
) -> Vec<Node> {
    match procedure {
        ArgSolveProcedure::Bfs => amazeing::matrix::solver::bfs(maze, from, to, tracer),
        ArgSolveProcedure::Dfs => amazeing::matrix::solver::dfs(maze, from, to, tracer),
        ArgSolveProcedure::Dijkstra => amazeing::matrix::solver::dijkstra(maze, from, to, tracer),
        ArgSolveProcedure::AStar => {
            amazeing::matrix::solver::a_star(maze, from, to, heuristic.unwrap(), tracer)
        }
    }
}

pub(crate) fn generate_maze(
    maze: &mut Maze,
    from: Vec<Node>,
    procedure: &ArgGenProcedure,
    tracer: &mut Option<Tracer>,
) {
    match procedure {
        ArgGenProcedure::Bfs => amazeing::matrix::generator::bfs(maze, from, tracer),
        ArgGenProcedure::Dfs => amazeing::matrix::generator::dfs(maze, from, tracer),
    }
}
