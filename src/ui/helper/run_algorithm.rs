use crate::command::{AmazeingContext, ArgProcedure};
use crate::core::tiled::node::WeightDirection;
use crate::core::tiled::{
    DNodeWeightedBackward, DNodeWeightedForward, Maze, Node, NodeHeuFn, Trace, Tracer, generator, solver,
};
use std::time::Instant;

pub(crate) fn solve_maze(
    maze: &Maze,
    source: Node,
    destination: Node,
    procedure: &ArgProcedure,
    heuristic: NodeHeuFn,
    tracer: &mut Option<Tracer>,
) -> Vec<Node> {
    let start = Instant::now();
    let solution = match procedure {
        ArgProcedure::Bfs => solver::bfs(maze, source, destination, tracer),
        ArgProcedure::Dfs => solver::dfs(maze, source, destination, tracer),
        ArgProcedure::AStar => solver::a_star(maze, source, destination, heuristic, tracer),
    };
    println!("Solved maze in {:?}", start.elapsed());
    solution
}

pub(crate) fn solve_maze_stream(
    maze: &Maze,
    source: Node,
    destination: Node,
    procedure: &ArgProcedure,
    heuristic: NodeHeuFn,
    emit: &mut dyn FnMut(Trace),
) -> Vec<Node> {
    let start = Instant::now();
    let solution = match procedure {
        ArgProcedure::Bfs => solver::bfs_stream(maze, source, destination, emit),
        ArgProcedure::Dfs => solver::dfs_stream(maze, source, destination, emit),
        ArgProcedure::AStar => solver::a_star_stream(maze, source, destination, heuristic, emit),
    };
    println!("Solved maze in {:?}", start.elapsed());
    solution
}

pub(crate) fn generate_maze(
    maze: &mut Maze,
    sources: &[Node],
    destination: Option<Node>,
    context: &AmazeingContext,
    tracer: &mut Option<Tracer>,
) {
    let start = Instant::now();
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
    println!("generated {} maze in {:?}", sources.len(), start.elapsed());
}

pub(crate) fn generate_maze_stream(
    maze: &mut Maze,
    sources: &[Node],
    destination: Option<Node>,
    context: &AmazeingContext,
    emit: &mut dyn FnMut(Trace),
) {
    let start = Instant::now();
    match context.procedure {
        ArgProcedure::Bfs => generator::bfs_stream(maze, sources, emit),
        ArgProcedure::Dfs => generator::dfs_stream(maze, sources, emit),
        ArgProcedure::AStar => {
            let a_star_fn = match context.weight_direction {
                WeightDirection::Backward => generator::a_star_stream::<DNodeWeightedBackward>,
                _ => generator::a_star_stream::<DNodeWeightedForward>,
            };
            a_star_fn(maze, sources, destination.unwrap(), context.heuristic, context.jumble_factor, emit)
        }
    }
    println!("generated {} maze in {:?}", sources.len(), start.elapsed());
}

