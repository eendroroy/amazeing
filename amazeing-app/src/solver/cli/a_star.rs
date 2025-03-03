use crate::solver::cli::formatter;
use crate::solver::matrix::cli_viz::CliViz;

use crate::{FROM, HEURISTIC, MAZE_DATA, TO};
use amazeing::solver::matrix::{
    a_star, chebyshev_heuristic, dijkstra_heuristic, euclidean_heuristic, manhattan_heuristic,
    octile_heuristic, Maze,
};

pub fn visualize() {
    let (maze, from, to) = (
        Maze::from(MAZE_DATA.lock().unwrap().clone()),
        FROM.lock().unwrap().clone(),
        TO.lock().unwrap().clone(),
    );

    let mut viz = CliViz::from_maze(&maze, '█', '█', '█', formatter::formatter);

    viz.merge_maze(&maze);

    let heu = match &*HEURISTIC.lock().unwrap().clone() {
        "manhattan" => manhattan_heuristic,
        "euclidean" => euclidean_heuristic,
        "chebyshev" => chebyshev_heuristic,
        "octile" => octile_heuristic,
        "dijkstra" => dijkstra_heuristic,
        other => panic!("Invalid heuristic function {}", other),
    };

    println!(
        "{}",
        viz.merged_path(a_star(&maze, from, to, heu, &mut None))
    );
}
