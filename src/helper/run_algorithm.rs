use crate::_lib::tiled::node::WeightDirection;
use crate::_lib::tiled::{
    DNodeWeightedBackward, DNodeWeightedForward, Maze, Node, NodeHeuFn, Tracer, UnitShape, generator, solver,
};
use crate::command::{ArgGenProcedure, ArgSolveProcedure};
use crate::context::CreateContext;

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
        ArgSolveProcedure::Bfs => solver::bfs(maze, unit_shape, source, destination, tracer),
        ArgSolveProcedure::Dfs => solver::dfs(maze, unit_shape, source, destination, tracer),
        ArgSolveProcedure::AStar => solver::a_star(maze, unit_shape, source, destination, heuristic, tracer),
    }
}

pub(crate) fn generate_maze(
    maze: &mut Maze,
    unit_shape: &UnitShape,
    sources: &[Node],
    destination: Option<Node>,
    context: &CreateContext,
    tracer: &mut Option<Tracer>,
) {
    match context.procedure {
        ArgGenProcedure::Bfs => generator::bfs(maze, unit_shape, sources, tracer),
        ArgGenProcedure::Dfs => generator::dfs(maze, unit_shape, sources, tracer),
        ArgGenProcedure::AStar => {
            let a_star_fn = match context.weight_direction {
                WeightDirection::Backward => generator::a_star::<DNodeWeightedBackward>,
                _ => generator::a_star::<DNodeWeightedForward>,
            };
            a_star_fn(maze, unit_shape, sources, destination.unwrap(), context.heuristic, context.jumble_factor, tracer)
        }
    }
}
