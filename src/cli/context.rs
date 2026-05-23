use crate::cli::{ArgEffect, ArgHeuristic, ArgProcedure, ArgWeightDirection};
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

// ── effect options ─────────────────────────────────────────────────────────────

/// Groups all optional visual-effect toggles into a single value.
#[derive(Debug, Clone, Default)]
pub struct EffectOptions {
    pub(crate) light_source: bool,
    pub(crate) fisheye: bool,
    pub(crate) color_source: bool,
    pub(crate) shockwave: bool,
}

impl EffectOptions {
    /// All effects disabled.
    pub fn none() -> Self {
        Self::default()
    }

    /// Build from the CLI effect list.
    pub fn from_args(effects: &[ArgEffect]) -> Self {
        Self {
            light_source: effects.contains(&ArgEffect::LightSource),
            fisheye: effects.contains(&ArgEffect::FishEye),
            color_source: effects.contains(&ArgEffect::ColorSource),
            shockwave: effects.contains(&ArgEffect::ShockwaveDistortion),
        }
    }
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
    pub(crate) effects: EffectOptions,
    pub(crate) context_type: ContextType,
}

impl AmazeingContext {
    pub fn create_context(
        (maze, maze_file_path): (Option<Maze>, Option<PathBuf>),
        (procedure, heuristic): (ArgProcedure, NodeHeuFn),
        (jumble_factor, weight_direction): (u32, WeightDirection),
        (rows, cols): (usize, usize),
        (zoom, fps, show_perimeter): (f32, f32, bool),
        effects: EffectOptions,
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
            effects,
            context_type: ContextType::Create,
        }
    }

    pub(crate) fn view_context(
        (maze, maze_file_path): (Maze, PathBuf),
        (zoom, fps, show_perimeter): (f32, f32, bool),
    ) -> Self {
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
            effects: EffectOptions::none(),
            context_type: ContextType::View,
        }
    }

    pub fn solve_context(
        maze: Maze,
        (procedure, heuristic): (ArgProcedure, NodeHeuFn),
        (zoom, fps, show_perimeter): (f32, f32, bool),
        effects: EffectOptions,
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
            effects,
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
            (None, Some(PathBuf::from("tmp/test.maze"))),
            (ArgProcedure::AStar, ArgHeuristic::Octile.heuristic()),
            (3, ArgWeightDirection::Forward.direction()),
            (21, 31),
            (1.25, 48.0, true),
            EffectOptions::none(),
        );
        assert_eq!(context.rows, 21);
        assert_eq!(context.cols, 31);
        assert_eq!(context.jumble_factor, 3);
        assert!(context.show_perimeter);
        assert!(!context.effects.light_source);
        assert!(!context.effects.fisheye);
        assert!(!context.effects.color_source);
        assert!(!context.effects.shockwave);
        assert_eq!(context.context_type, ContextType::Create);
        assert_eq!(context.maze_file_path, Some(PathBuf::from("tmp/test.maze")));
    }

    #[test]
    fn view_and_solve_context_copy_maze_dimensions() {
        let maze = Maze::new_void(UnitShape::Square, 5, 7);

        let vc = AmazeingContext::view_context((maze.clone(), PathBuf::from("tmp/view.maze")), (1.0, 60.0, false));
        assert_eq!(vc.rows, 5);
        assert_eq!(vc.cols, 7);
        assert_eq!(vc.context_type, ContextType::View);

        let all_effects = EffectOptions { light_source: true, fisheye: true, color_source: true, shockwave: true };
        let sc = AmazeingContext::solve_context(
            maze,
            (ArgProcedure::Dfs, ArgHeuristic::Dijkstra.heuristic()),
            (1.0, 60.0, false),
            all_effects,
        );
        assert_eq!(sc.rows, 5);
        assert_eq!(sc.cols, 7);
        assert!(sc.effects.light_source);
        assert!(sc.effects.fisheye);
        assert!(sc.effects.color_source);
        assert!(sc.effects.shockwave);
        assert_eq!(sc.context_type, ContextType::Solve);
    }
}
