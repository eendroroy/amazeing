use crate::command::{ArgGenProcedure, ArgSolveProcedure};
use amazeing::tiled::{Maze, Node, NodeHeuFn, Tracer, UnitShape};

pub(crate) fn solve_maze(
    maze: &Maze,
    unit_shape: &UnitShape,
    source: Node,
    destination: Node,
    procedure: &ArgSolveProcedure,
    heuristic: NodeHeuFn,
    tracer: &mut Option<Tracer>,
) -> Vec<Node> {
    match procedure {
        ArgSolveProcedure::Bfs => amazeing::tiled::solver::bfs(maze, unit_shape, source, destination, tracer),
        ArgSolveProcedure::Dfs => amazeing::tiled::solver::dfs(maze, unit_shape, source, destination, tracer),
        ArgSolveProcedure::Dijkstra => amazeing::tiled::solver::dijkstra(maze, unit_shape, source, destination, tracer),
        ArgSolveProcedure::AStar => {
            amazeing::tiled::solver::a_star(maze, unit_shape, source, destination, heuristic, tracer)
        }
    }
}

pub(crate) fn generate_maze(
    maze: &mut Maze,
    unit_shape: &UnitShape,
    sources: &[Node],
    destination: Option<Node>,
    procedure: &ArgGenProcedure,
    heuristic: NodeHeuFn,
    jumble_factor: u32,
    tracer: &mut Option<Tracer>,
) {
    match procedure {
        ArgGenProcedure::Bfs => amazeing::tiled::generator::bfs(maze, unit_shape, sources, tracer),
        ArgGenProcedure::Dfs => amazeing::tiled::generator::dfs(maze, unit_shape, sources, tracer),
        ArgGenProcedure::AStar => amazeing::tiled::generator::a_star(
            maze,
            unit_shape,
            sources,
            destination.unwrap(),
            heuristic,
            jumble_factor,
            tracer,
        ),
    }
}
