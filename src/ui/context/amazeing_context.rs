use crate::command::{ArgGenProcedure, ArgHeuristic, ArgSolveProcedure, ArgWeightDirection};
use crate::core::tiled::node::WeightDirection;
use crate::core::tiled::{Maze, NodeHeuFn};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ContextType {
    Create,
    View,
    Solve,
}

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
    pub(crate) zoom: f32,
    pub(crate) fps: f32,
    pub(crate) show_perimeter: bool,
    pub(crate) context_type: ContextType,
}

impl AmazeingContext {
    pub fn create_context(
        (maze, maze_file_path): (Option<Maze>, Option<PathBuf>),
        (generation_procedure, heuristic): (ArgGenProcedure, NodeHeuFn),
        (jumble_factor, weight_direction): (u32, WeightDirection),
        (rows, cols): (usize, usize),
        (zoom, fps, show_perimeter): (f32, f32, bool),
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
            zoom,
            fps,
            show_perimeter,
            context_type: ContextType::Create,
        }
    }

    pub(crate) fn view_context(
        (maze, maze_file_path): (Maze, PathBuf),
        (zoom, fps, show_perimeter): (f32, f32, bool),
    ) -> Self {
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
            zoom,
            fps,
            show_perimeter,
            context_type: ContextType::View,
        }
    }

    pub fn solve_context(
        maze: Maze,
        (solve_procedure, heuristic): (ArgSolveProcedure, NodeHeuFn),
        (zoom, fps, show_perimeter): (f32, f32, bool),
    ) -> Self {
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
            zoom,
            fps,
            show_perimeter,
            context_type: ContextType::Solve,
        }
    }
}
