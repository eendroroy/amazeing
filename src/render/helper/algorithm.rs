use crate::cli::{AmazeingContext, ArgProcedure};
use crate::maze::node::WeightDirection;
use crate::maze::{
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::{ArgHeuristic, ArgWeightDirection};
    use crate::maze::heuristics::manhattan_heuristic;
    use crate::maze::{BLOCK, NodeFactory, OPEN, UnitShape};

    #[test]
    fn solve_dispatch_works_for_all_procedures() {
        let maze = Maze::new(UnitShape::Square, 3, 3, OPEN);
        let f = NodeFactory::new(3, 3);
        let s = f.at(0, 0).unwrap();
        let d = f.at(2, 2).unwrap();

        for p in [ArgProcedure::Bfs, ArgProcedure::Dfs, ArgProcedure::AStar] {
            let path = solve_maze(&maze, s, d, &p, manhattan_heuristic, &mut None);
            assert_eq!(path.first(), Some(&s));
            assert_eq!(path.last(), Some(&d));

            let mut emitted = 0usize;
            let mut emit = |_| emitted += 1;
            let stream_path = solve_maze_stream(&maze, s, d, &p, manhattan_heuristic, &mut emit);
            assert!(!stream_path.is_empty());
            assert!(emitted > 0);
        }
    }

    #[test]
    fn generate_dispatch_works_for_all_procedures() {
        let mut maze = Maze::new(UnitShape::Square, 5, 5, BLOCK);
        let f = NodeFactory::new(5, 5);
        let source = f.at(2, 2).unwrap();
        let destination = f.at(4, 4).unwrap();

        let bfs_ctx = AmazeingContext::create_context(
            (None, None),
            (ArgProcedure::Bfs, ArgHeuristic::Dijkstra.heuristic()),
            (0, ArgWeightDirection::Forward.direction()),
            (5, 5),
            (1.0, 60.0, false),
        );
        generate_maze(&mut maze, &[source], None, &bfs_ctx, &mut None);
        assert_eq!(maze[source], OPEN);

        let mut maze_dfs = Maze::new(UnitShape::Square, 5, 5, BLOCK);
        let dfs_ctx = AmazeingContext::create_context(
            (None, None),
            (ArgProcedure::Dfs, ArgHeuristic::Dijkstra.heuristic()),
            (0, ArgWeightDirection::Forward.direction()),
            (5, 5),
            (1.0, 60.0, false),
        );
        generate_maze_stream(&mut maze_dfs, &[source], None, &dfs_ctx, &mut |_| {});
        assert_eq!(maze_dfs[source], OPEN);

        let mut maze_astar = Maze::new(UnitShape::Square, 5, 5, BLOCK);
        let astar_ctx = AmazeingContext::create_context(
            (None, None),
            (ArgProcedure::AStar, manhattan_heuristic),
            (0, ArgWeightDirection::Backward.direction()),
            (5, 5),
            (1.0, 60.0, false),
        );
        generate_maze(&mut maze_astar, &[source], Some(destination), &astar_ctx, &mut None);
        assert_eq!(maze_astar[source], OPEN);
    }
}
