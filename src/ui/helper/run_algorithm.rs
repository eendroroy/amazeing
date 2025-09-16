use crate::command::{AmazeingContext, ArgProcedure};
use crate::core::tiled::node::WeightDirection;
use crate::core::tiled::{
    DNodeWeightedBackward, DNodeWeightedForward, Maze, Node, NodeHeuFn, Tracer, generator, solver,
};

pub(crate) fn solve_maze(
    maze: &Maze,
    source: Node,
    destination: Node,
    procedure: &ArgProcedure,
    heuristic: NodeHeuFn,
    tracer: &mut Option<Tracer>,
) -> Vec<Node> {
    match procedure {
        ArgProcedure::Bfs => solver::bfs(maze, source, destination, tracer),
        ArgProcedure::Dfs => solver::dfs(maze, source, destination, tracer),
        ArgProcedure::AStar => solver::a_star(maze, source, destination, heuristic, tracer),
    }
}

pub(crate) fn generate_maze(
    maze: &mut Maze,
    sources: &[Node],
    destination: Option<Node>,
    context: &AmazeingContext,
    tracer: &mut Option<Tracer>,
) {
    match context.procedure {
        ArgProcedure::Bfs => generator::bfs(maze, sources, tracer),
        ArgProcedure::Dfs => generator::dfs(maze, sources, tracer),
        ArgProcedure::AStar => {
            let a_star_fn = match context.weight_direction {
                WeightDirection::Backward => generator::a_star::<DNodeWeightedBackward>,
                _ => generator::a_star::<DNodeWeightedForward>,
            };
            a_star_fn(maze, sources, destination.unwrap(), context.heuristic, context.jumble_factor, tracer)
        }
    }
}
