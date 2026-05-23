use crate::cli::{ArgHeuristic, ArgProcedure, ArgWeightDirection};
use crate::maze::node::WeightDirection;
use crate::maze::{Maze, NodeHeuFn};
use std::path::PathBuf;

// ── context type ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ContextType {
    Create,
    View,
    Solve,
}

// ── runtime context ───────────────────────────────────────────────────────────

/// Carries all resolved runtime parameters for a single command invocation.
#[derive(Debug, Clone)]
pub struct AmazeingContext {
    pub(crate) maze: Option<Maze>,
    pub(crate) maze_file_path: Option<PathBuf>,
    pub(crate) procedure: ArgProcedure,
    pub(crate) heuristic: NodeHeuFn,
    pub(crate) jumble_factor: u32,
    pub(crate) weight_direction: WeightDirection,
    pub(crate) rows: usize,
    pub(crate) cols: usize,
    pub(crate) zoom: f32,
    pub(crate) fps: f32,
    pub(crate) show_perimeter: bool,
    pub(crate) light_source_effect: bool,
    pub(crate) context_type: ContextType,
}

impl AmazeingContext {
    #[allow(clippy::too_many_arguments)]
    pub fn create_context(
        maze: Option<Maze>,
        maze_file_path: Option<PathBuf>,
        procedure: ArgProcedure,
        heuristic: NodeHeuFn,
        jumble_factor: u32,
        weight_direction: WeightDirection,
        rows: usize,
        cols: usize,
        zoom: f32,
        fps: f32,
        show_perimeter: bool,
        light_source_effect: bool,
    ) -> Self {
        Self {
            maze,
            maze_file_path,
            procedure,
            heuristic,
            jumble_factor,
            weight_direction,
            rows,
            cols,
            zoom,
            fps,
            show_perimeter,
            light_source_effect,
            context_type: ContextType::Create,
        }
    }

    pub(crate) fn view_context(maze: Maze, maze_file_path: PathBuf, zoom: f32, fps: f32, show_perimeter: bool) -> Self {
        Self {
            rows: maze.rows(),
            cols: maze.cols(),
            maze_file_path: Some(maze_file_path),
            procedure: ArgProcedure::default(),
            heuristic: ArgHeuristic::default().heuristic(),
            jumble_factor: 0,
            weight_direction: ArgWeightDirection::default().direction(),
            maze: Some(maze),
            zoom,
            fps,
            show_perimeter,
            light_source_effect: false,
            context_type: ContextType::View,
        }
    }

    pub fn solve_context(
        maze: Maze,
        procedure: ArgProcedure,
        heuristic: NodeHeuFn,
        zoom: f32,
        fps: f32,
        show_perimeter: bool,
        light_source_effect: bool,
    ) -> Self {
        Self {
            rows: maze.rows(),
            cols: maze.cols(),
            maze: Some(maze),
            maze_file_path: None,
            procedure,
            heuristic,
            jumble_factor: 0,
            weight_direction: ArgWeightDirection::default().direction(),
            zoom,
            fps,
            show_perimeter,
            light_source_effect,
            context_type: ContextType::Solve,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::maze::{Maze, UnitShape};

    #[test]
    fn create_context_sets_expected_fields() {
        let context = AmazeingContext::create_context(
            None,
            Some(PathBuf::from("tmp/test.maze")),
            ArgProcedure::AStar,
            ArgHeuristic::Octile.heuristic(),
            3,
            ArgWeightDirection::Forward.direction(),
            21,
            31,
            1.25,
            48.0,
            true,
            false,
        );
        assert_eq!(context.rows, 21);
        assert_eq!(context.cols, 31);
        assert_eq!(context.jumble_factor, 3);
        assert!(context.show_perimeter);
        assert!(!context.light_source_effect);
        assert_eq!(context.context_type, ContextType::Create);
        assert_eq!(context.maze_file_path, Some(PathBuf::from("tmp/test.maze")));
    }

    #[test]
    fn view_and_solve_context_copy_maze_dimensions() {
        let maze = Maze::new_void(UnitShape::Square, 5, 7);

        let vc = AmazeingContext::view_context(maze.clone(), PathBuf::from("tmp/view.maze"), 1.0, 60.0, false);
        assert_eq!(vc.rows, 5);
        assert_eq!(vc.cols, 7);
        assert_eq!(vc.context_type, ContextType::View);

        let sc = AmazeingContext::solve_context(
            maze,
            ArgProcedure::Dfs,
            ArgHeuristic::Dijkstra.heuristic(),
            1.0,
            60.0,
            false,
            true,
        );
        assert_eq!(sc.rows, 5);
        assert_eq!(sc.cols, 7);
        assert!(sc.light_source_effect);
        assert_eq!(sc.context_type, ContextType::Solve);
    }
}
