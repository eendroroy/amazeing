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
        ArgProcedure::Prim => panic!("Prim is only available for maze generation, not solving."),
        ArgProcedure::BeamSearch => solver::beam_search(maze, source, destination, heuristic, tracer),
        ArgProcedure::Iddfs => solver::iddfs(maze, source, destination, tracer),
        ArgProcedure::GreedyBestFirst => solver::greedy_best_first(maze, source, destination, heuristic, tracer),
        ArgProcedure::BidirectionalBfs => solver::bidirectional_bfs(maze, source, destination, tracer),
        ArgProcedure::BidirectionalGreedyBestFirst => {
            solver::bidirectional_greedy_best_first(maze, source, destination, heuristic, tracer)
        }
        ArgProcedure::SimulatedAnnealingSearch => {
            solver::simulated_annealing_search(maze, source, destination, heuristic, tracer)
        }
        ArgProcedure::AldousBroder => solver::aldous_broder(maze, source, destination, tracer),
        ArgProcedure::BidirectionalAStart => {
            solver::bidirectional_a_start(maze, source, destination, heuristic, tracer)
        }
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
        ArgProcedure::Prim => panic!("Prim is only available for maze generation, not solving."),
        ArgProcedure::BeamSearch => solver::beam_search_stream(maze, source, destination, heuristic, emit),
        ArgProcedure::Iddfs => solver::iddfs_stream(maze, source, destination, emit),
        ArgProcedure::GreedyBestFirst => solver::greedy_best_first_stream(maze, source, destination, heuristic, emit),
        ArgProcedure::BidirectionalBfs => solver::bidirectional_bfs_stream(maze, source, destination, emit),
        ArgProcedure::BidirectionalGreedyBestFirst => {
            solver::bidirectional_greedy_best_first_stream(maze, source, destination, heuristic, emit)
        }
        ArgProcedure::SimulatedAnnealingSearch => {
            solver::simulated_annealing_search_stream(maze, source, destination, heuristic, emit)
        }
        ArgProcedure::AldousBroder => solver::aldous_broder_stream(maze, source, destination, emit),
        ArgProcedure::BidirectionalAStart => {
            solver::bidirectional_a_start_stream(maze, source, destination, heuristic, emit)
        }
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
        ArgProcedure::Prim => generator::prim(maze, sources, tracer),
        ArgProcedure::BeamSearch => {
            let target = destination.unwrap_or_else(|| *sources.first().expect("at least one source is required"));
            generator::beam_search(maze, sources, target, context.heuristic, context.jumble_factor, tracer)
        }
        ArgProcedure::Iddfs => generator::iddfs(maze, sources, tracer),
        ArgProcedure::GreedyBestFirst => {
            panic!("Greedy Best-First is only available for maze solving, not generation.")
        }
        ArgProcedure::BidirectionalBfs => {
            panic!("Bidirectional BFS is only available for maze solving, not generation.")
        }
        ArgProcedure::BidirectionalGreedyBestFirst => {
            let target = destination.unwrap_or_else(|| *sources.first().expect("at least one source is required"));
            generator::bidirectional_greedy_best_first(
                maze,
                sources,
                target,
                context.heuristic,
                context.jumble_factor,
                tracer,
            )
        }
        ArgProcedure::SimulatedAnnealingSearch => {
            generator::simulated_annealing_search(maze, sources, destination, context.heuristic, tracer)
        }
        ArgProcedure::AldousBroder => generator::aldous_broder(maze, sources, tracer),
        ArgProcedure::BidirectionalAStart => generator::bidirectional_a_start(
            maze,
            sources,
            destination.unwrap(),
            context.heuristic,
            context.jumble_factor,
            tracer,
        ),
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
        ArgProcedure::Prim => generator::prim_stream(maze, sources, emit),
        ArgProcedure::BeamSearch => {
            let target = destination.unwrap_or_else(|| *sources.first().expect("at least one source is required"));
            generator::beam_search_stream(maze, sources, target, context.heuristic, context.jumble_factor, emit)
        }
        ArgProcedure::Iddfs => generator::iddfs_stream(maze, sources, emit),
        ArgProcedure::GreedyBestFirst => {
            panic!("Greedy Best-First is only available for maze solving, not generation.")
        }
        ArgProcedure::BidirectionalBfs => {
            panic!("Bidirectional BFS is only available for maze solving, not generation.")
        }
        ArgProcedure::BidirectionalGreedyBestFirst => {
            let target = destination.unwrap_or_else(|| *sources.first().expect("at least one source is required"));
            generator::bidirectional_greedy_best_first_stream(
                maze,
                sources,
                target,
                context.heuristic,
                context.jumble_factor,
                emit,
            )
        }
        ArgProcedure::SimulatedAnnealingSearch => {
            generator::simulated_annealing_search_stream(maze, sources, destination, context.heuristic, emit)
        }
        ArgProcedure::AldousBroder => generator::aldous_broder_stream(maze, sources, emit),
        ArgProcedure::BidirectionalAStart => generator::bidirectional_a_start_stream(
            maze,
            sources,
            destination.unwrap(),
            context.heuristic,
            context.jumble_factor,
            emit,
        ),
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

        for p in [
            ArgProcedure::Bfs,
            ArgProcedure::Dfs,
            ArgProcedure::BeamSearch,
            ArgProcedure::Iddfs,
            ArgProcedure::GreedyBestFirst,
            ArgProcedure::BidirectionalBfs,
            ArgProcedure::BidirectionalGreedyBestFirst,
            ArgProcedure::SimulatedAnnealingSearch,
            ArgProcedure::AldousBroder,
            ArgProcedure::BidirectionalAStart,
            ArgProcedure::AStar,
        ] {
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
        let f = NodeFactory::new(5, 5);
        let source = f.at(2, 2).unwrap();
        let destination = f.at(4, 4).unwrap();

        let make_ctx = |proc, heu| {
            AmazeingContext::create_context(None, None, proc, heu, 0, ArgWeightDirection::Forward.direction(), 5, 5, 1.0, 60.0, false)
        };

        let mut maze = Maze::new(UnitShape::Square, 5, 5, BLOCK);
        generate_maze(&mut maze, &[source], None, &make_ctx(ArgProcedure::Bfs, ArgHeuristic::Dijkstra.heuristic()), &mut None);
        assert_eq!(maze[source], OPEN);

        let mut maze_dfs = Maze::new(UnitShape::Square, 5, 5, BLOCK);
        generate_maze_stream(&mut maze_dfs, &[source], None, &make_ctx(ArgProcedure::Dfs, ArgHeuristic::Dijkstra.heuristic()), &mut |_| {});
        assert_eq!(maze_dfs[source], OPEN);

        let mut maze_prim = Maze::new(UnitShape::Square, 5, 5, BLOCK);
        generate_maze_stream(&mut maze_prim, &[source], None, &make_ctx(ArgProcedure::Prim, ArgHeuristic::Dijkstra.heuristic()), &mut |_| {});
        assert_eq!(maze_prim[source], OPEN);

        let mut maze_beam = Maze::new(UnitShape::Square, 5, 5, BLOCK);
        generate_maze_stream(&mut maze_beam, &[source], Some(destination), &make_ctx(ArgProcedure::BeamSearch, manhattan_heuristic), &mut |_| {});
        assert_eq!(maze_beam[source], OPEN);

        let mut maze_iddfs = Maze::new(UnitShape::Square, 5, 5, BLOCK);
        generate_maze_stream(&mut maze_iddfs, &[source], None, &make_ctx(ArgProcedure::Iddfs, ArgHeuristic::Dijkstra.heuristic()), &mut |_| {});
        assert_eq!(maze_iddfs[source], OPEN);

        let mut maze_ab = Maze::new(UnitShape::Square, 5, 5, BLOCK);
        generate_maze_stream(&mut maze_ab, &[source], None, &make_ctx(ArgProcedure::AldousBroder, ArgHeuristic::Dijkstra.heuristic()), &mut |_| {});
        assert_eq!(maze_ab[source], OPEN);

        let mut maze_bi = Maze::new(UnitShape::Square, 5, 5, BLOCK);
        generate_maze_stream(&mut maze_bi, &[source], Some(destination), &make_ctx(ArgProcedure::BidirectionalAStart, manhattan_heuristic), &mut |_| {});
        assert_eq!(maze_bi[source], OPEN);

        let mut maze_bi_gbf = Maze::new(UnitShape::Square, 5, 5, BLOCK);
        generate_maze_stream(&mut maze_bi_gbf, &[source], Some(destination), &make_ctx(ArgProcedure::BidirectionalGreedyBestFirst, manhattan_heuristic), &mut |_| {});
        assert_eq!(maze_bi_gbf[source], OPEN);

        let mut maze_sas = Maze::new(UnitShape::Square, 5, 5, BLOCK);
        generate_maze_stream(&mut maze_sas, &[source], Some(destination), &make_ctx(ArgProcedure::SimulatedAnnealingSearch, manhattan_heuristic), &mut |_| {});
        assert_eq!(maze_sas[source], OPEN);

        let mut maze_astar = Maze::new(UnitShape::Square, 5, 5, BLOCK);
        let astar_ctx = AmazeingContext::create_context(
            None, None,
            ArgProcedure::AStar, manhattan_heuristic,
            0, ArgWeightDirection::Backward.direction(),
            5, 5, 1.0, 60.0, false,
        );
        generate_maze(&mut maze_astar, &[source], Some(destination), &astar_ctx, &mut None);
        assert_eq!(maze_astar[source], OPEN);
    }
}

