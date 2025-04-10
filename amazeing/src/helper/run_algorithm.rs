use crate::command::{ArgGenProcedure, ArgSolveProcedure};
use amazeing::matrix::{Maze, Node, NodeHeuFn, Tracer, UnitShape};

pub(crate) fn solve_maze(
    maze: &Maze,
    unit_shape: &UnitShape,
    from: Node,
    to: Node,
    procedure: &ArgSolveProcedure,
    heuristic: Option<NodeHeuFn>,
    tracer: &mut Option<Tracer>,
) -> Vec<Node> {
    match procedure {
        ArgSolveProcedure::Bfs => amazeing::matrix::solver::bfs(maze, unit_shape, from, to, tracer),
        ArgSolveProcedure::Dfs => amazeing::matrix::solver::dfs(maze, unit_shape, from, to, tracer),
        ArgSolveProcedure::Dijkstra => amazeing::matrix::solver::dijkstra(maze, unit_shape, from, to, tracer),
        ArgSolveProcedure::AStar => {
            amazeing::matrix::solver::a_star(maze, unit_shape, from, to, heuristic.unwrap(), tracer)
        }
    }
}

pub(crate) fn generate_maze(
    maze: &mut Maze,
    unit_shape: &UnitShape,
    from: Vec<Node>,
    procedure: &ArgGenProcedure,
    tracer: &mut Option<Tracer>,
) {
    match procedure {
        ArgGenProcedure::Bfs => amazeing::matrix::generator::bfs(maze, unit_shape, from, tracer),
        ArgGenProcedure::Dfs => amazeing::matrix::generator::dfs(maze, unit_shape, from, tracer),
    }
}
