use crate::command::{ArgGenProcedure, ArgSolveProcedure};
use crate::core::tiled::node::WeightDirection;
use crate::core::tiled::{
    DNodeWeightedBackward, DNodeWeightedForward, Maze, Node, NodeHeuFn, Tracer, generator, solver,
};
use crate::ui::context::AmazeingContext;

pub(crate) fn solve_maze(
    maze: &Maze,
    source: Node,
    destination: Node,
    procedure: &ArgSolveProcedure,
    heuristic: NodeHeuFn,
    tracer: &mut Option<Tracer>,
) -> Vec<Node> {
    match procedure {
        ArgSolveProcedure::Bfs => solver::bfs(maze, source, destination, tracer),
        ArgSolveProcedure::Dfs => solver::dfs(maze, source, destination, tracer),
        ArgSolveProcedure::AStar => solver::a_star(maze, source, destination, heuristic, tracer),
    }
}

pub(crate) fn generate_maze(
    maze: &mut Maze,
    sources: &[Node],
    destination: Option<Node>,
    context: &AmazeingContext,
    tracer: &mut Option<Tracer>,
) {
    match context.generation_procedure {
        ArgGenProcedure::Bfs => generator::bfs(maze, sources, tracer),
        ArgGenProcedure::Dfs => generator::dfs(maze, sources, tracer),
        ArgGenProcedure::AStar => {
            let a_star_fn = match context.weight_direction {
                WeightDirection::Backward => generator::a_star::<DNodeWeightedBackward>,
                _ => generator::a_star::<DNodeWeightedForward>,
            };
            a_star_fn(maze, sources, destination.unwrap(), context.heuristic, context.jumble_factor, tracer)
        }
    }
}
