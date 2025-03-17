use crate::command::{ArgGenProcedure, ArgSolveProcedure};
use amazeing::maze::matrix::Maze;
use amazeing::{generator, solver, Node, NodeHeuFn, Tracer};

pub(crate) fn solve_maze(
    maze: &Maze,
    from: Node,
    to: Node,
    procedure: &ArgSolveProcedure,
    heuristic: Option<NodeHeuFn>,
    tracer: &mut Option<Tracer>,
) -> Vec<Node> {
    println!("{:?} ==> {:?}", from, to);
    match procedure {
        ArgSolveProcedure::Bfs => solver::matrix::bfs(maze, from, to, tracer),
        ArgSolveProcedure::Dfs => solver::matrix::dfs(maze, from, to, tracer),
        ArgSolveProcedure::Dijkstra => solver::matrix::dijkstra(maze, from, to, tracer),
        ArgSolveProcedure::AStar => {
            solver::matrix::a_star(maze, from, to, heuristic.unwrap(), tracer)
        }
    }
}

pub(crate) fn generate_maze(
    maze: &mut Maze,
    from: Node,
    procedure: &ArgGenProcedure,
    tracer: &mut Option<Tracer>,
) {
    match procedure {
        ArgGenProcedure::Bfs => generator::matrix::bfs(maze, from, tracer),
        ArgGenProcedure::Dfs => generator::matrix::dfs(maze, from, tracer),
    }
}
