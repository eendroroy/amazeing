use crate::cli::cli_viz::CliViz;
use crate::cli::formatter;
use amazeing::solver::matrix::Maze;

pub fn visualize(maze: &Maze, path: Vec<(usize, usize)>) {
    let mut viz = CliViz::from_maze(&maze, '█', '█', '█', formatter::formatter);

    viz.merge_maze(&maze);

    println!("{}", viz.merged_path(path));
}
