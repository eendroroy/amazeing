use crate::cli::formatter;
use crate::matrix::cli_viz::CliViz;
use crate::SOLVER_CONTEXT;
use amazeing::solver::matrix::{dijkstra, Maze};

pub fn visualize() {
    let (maze, from, to) = (
        Maze::from(SOLVER_CONTEXT.read().unwrap().maze_data.clone()),
        SOLVER_CONTEXT.read().unwrap().from,
        SOLVER_CONTEXT.read().unwrap().to,
    );

    let mut viz = CliViz::from_maze(&maze, '█', '█', '█', formatter::formatter);

    viz.merge_maze(&maze);

    println!("{}", viz.merged_path(dijkstra(&maze, from, to, &mut None)));
}
