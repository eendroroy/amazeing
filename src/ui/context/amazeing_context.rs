use crate::command::{ArgGenProcedure, ArgHeuristic, ArgSolveProcedure, ArgWeightDirection};
use crate::core::tiled::node::WeightDirection;
use crate::core::tiled::{Maze, NodeHeuFn};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct AmazeingContext {
    pub(crate) maze: Option<Maze>,
    pub(crate) maze_file_path: Option<PathBuf>,
    pub(crate) generation_procedure: ArgGenProcedure,
    pub(crate) solve_procedure: ArgSolveProcedure,
    pub(crate) heuristic: NodeHeuFn,
    pub(crate) jumble_factor: u32,
    pub(crate) weight_direction: WeightDirection,
    pub(crate) rows: usize,
    pub(crate) cols: usize,
}

impl AmazeingContext {
    pub fn create_context(
        maze: Option<Maze>,
        maze_file_path: Option<PathBuf>,
        generation_procedure: ArgGenProcedure,
        heuristic: NodeHeuFn,
        jumble_factor: u32,
        weight_direction: WeightDirection,
        (rows, cols): (usize, usize),
    ) -> Self {
        Self {
            maze,
            maze_file_path,
            generation_procedure,
            solve_procedure: Default::default(),
            heuristic,
            jumble_factor,
            weight_direction,
            rows,
            cols,
        }
    }

    pub(crate) fn view_context(maze: Maze, maze_file_path: PathBuf) -> Self {
        Self {
            maze_file_path: Some(maze_file_path),
            generation_procedure: ArgGenProcedure::default(),
            solve_procedure: ArgSolveProcedure::default(),
            heuristic: ArgHeuristic::default().heuristic(),
            jumble_factor: 0,
            weight_direction: ArgWeightDirection::default().direction(),
            rows: maze.rows(),
            cols: maze.cols(),
            maze: Some(maze),
        }
    }

    pub fn solve_context(maze: Maze, solve_procedure: ArgSolveProcedure, heuristic: NodeHeuFn) -> Self {
        Self {
            maze_file_path: None,
            generation_procedure: ArgGenProcedure::default(),
            solve_procedure,
            heuristic,
            jumble_factor: 0,
            weight_direction: ArgWeightDirection::default().direction(),
            rows: maze.rows(),
            cols: maze.cols(),
            maze: Some(maze),
        }
    }
}
