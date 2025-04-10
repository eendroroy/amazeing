use crate::command::{ArgGenProcedure, ArgSolveProcedure};
use amazeing::matrix::{Maze, Node, NodeHeuFn, Tracer, UnitShape};

pub(crate) fn solve_maze(
    maze: &Maze,
    shape: &UnitShape,
    from: Node,
    to: Node,
    procedure: &ArgSolveProcedure,
    heuristic: Option<NodeHeuFn>,
    tracer: &mut Option<Tracer>,
) -> Vec<Node> {
    match procedure {
        ArgSolveProcedure::Bfs => amazeing::matrix::solver::bfs(maze, shape, from, to, tracer),
        ArgSolveProcedure::Dfs => amazeing::matrix::solver::dfs(maze, shape, from, to, tracer),
        ArgSolveProcedure::Dijkstra => amazeing::matrix::solver::dijkstra(maze, shape, from, to, tracer),
        ArgSolveProcedure::AStar => amazeing::matrix::solver::a_star(maze, shape, from, to, heuristic.unwrap(), tracer),
    }
}

pub(crate) fn generate_maze(
    maze: &mut Maze,
    shape: &UnitShape,
    from: Vec<Node>,
    procedure: &ArgGenProcedure,
    tracer: &mut Option<Tracer>,
) {
    match procedure {
        ArgGenProcedure::Bfs => amazeing::matrix::generator::bfs(maze, shape, from, tracer),
        ArgGenProcedure::Dfs => amazeing::matrix::generator::dfs(maze, shape, from, tracer),
    }
}
