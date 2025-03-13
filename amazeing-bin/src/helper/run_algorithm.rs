use crate::command::{ArgGenProcedure, ArgSolveProcedure};
use amazeing::maze::matrix::Maze;
use amazeing::{generator, solver, DNode, HeuFn};

pub(crate) fn solve_maze(
    maze: &Maze,
    from: DNode,
    to: DNode,
    procedure: &ArgSolveProcedure,
    heuristic: Option<HeuFn>,
    tracer: &mut Option<Vec<Vec<DNode>>>,
) -> Vec<DNode> {
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
    from: DNode,
    procedure: &ArgGenProcedure,
    tracer: &mut Option<Vec<Vec<DNode>>>,
) {
    match procedure {
        ArgGenProcedure::Random => generator::matrix::random(maze, from, tracer),
        ArgGenProcedure::Dfs => generator::matrix::dfs(maze, from, tracer),
    }
}
