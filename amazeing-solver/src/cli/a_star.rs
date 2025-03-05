use crate::cli::formatter;
use crate::matrix::cli_viz::CliViz;
use crate::{FROM, HEURISTIC, MAZE_DATA, TO};
use amazeing::solver::matrix::{a_star, Maze};

pub fn visualize() {
    let (maze, from, to) = (
        Maze::from(MAZE_DATA.lock().unwrap().clone()),
        FROM.lock().unwrap().clone(),
        TO.lock().unwrap().clone(),
    );

    let mut viz = CliViz::from_maze(&maze, '█', '█', '█', formatter::formatter);

    viz.merge_maze(&maze);

    println!(
        "{}",
        viz.merged_path(a_star(
            &maze,
            from,
            to,
            *HEURISTIC.lock().unwrap(),
            &mut None
        ))
    );
}
