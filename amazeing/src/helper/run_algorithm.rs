use crate::command::{ArgGenProcedure, ArgSolveProcedure};
use amazeing::tiled::{Maze, Node, NodeHeuFn, Tracer, UnitShape};

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
        ArgSolveProcedure::Bfs => amazeing::tiled::solver::bfs(maze, unit_shape, from, to, tracer),
        ArgSolveProcedure::Dfs => amazeing::tiled::solver::dfs(maze, unit_shape, from, to, tracer),
        ArgSolveProcedure::Dijkstra => amazeing::tiled::solver::dijkstra(maze, unit_shape, from, to, tracer),
        ArgSolveProcedure::AStar => {
            amazeing::tiled::solver::a_star(maze, unit_shape, from, to, heuristic.unwrap(), tracer)
        }
    }
}

pub(crate) fn generate_maze(
    maze: &mut Maze,
    unit_shape: &UnitShape,
    sources: &[Node],
    procedure: &ArgGenProcedure,
    tracer: &mut Option<Tracer>,
) {
    match procedure {
        ArgGenProcedure::Bfs => amazeing::tiled::generator::bfs(maze, unit_shape, sources, tracer),
        ArgGenProcedure::Dfs => amazeing::tiled::generator::dfs(maze, unit_shape, sources, tracer),
    }
}
