use crate::cli::{ArgHeuristic, ArgProcedure, ArgWeightDirection};
use crate::maze::tiled::node::WeightDirection;
use crate::maze::tiled::{Maze, NodeHeuFn, Rank};
use crate::render::helper::gradient;
use macroquad::color::Color;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

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
    pub(crate) procedure: ArgProcedure,
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
        (procedure, heuristic): (ArgProcedure, NodeHeuFn),
        (jumble_factor, weight_direction): (u32, WeightDirection),
        (rows, cols): (usize, usize),
        (zoom, fps, show_perimeter): (f32, f32, bool),
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
            context_type: ContextType::Create,
        }
    }

    pub(crate) fn view_context(
        (maze, maze_file_path): (Maze, PathBuf),
        (zoom, fps, show_perimeter): (f32, f32, bool),
    ) -> Self {
        Self {
            maze_file_path: Some(maze_file_path),
            procedure: ArgProcedure::default(),
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
        (procedure, heuristic): (ArgProcedure, NodeHeuFn),
        (zoom, fps, show_perimeter): (f32, f32, bool),
    ) -> Self {
        Self {
            maze_file_path: None,
            procedure,
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

#[derive(Debug, Clone)]
pub struct Colors {
    pub(crate) color_bg: Color,
    pub(crate) color_block: Color,
    pub(crate) color_open: Color,
    pub(crate) color_visiting: Color,
    pub(crate) color_visiting_gradient: Vec<Color>,
    pub(crate) color_path: Color,
    pub(crate) color_source: Color,
    pub(crate) color_destination: Color,
    pub(crate) color_traversed: Color,
    pub(crate) color_perimeter: Color,
}

impl Colors {
    pub fn new(steps: usize) -> Self {
        Self {
            color_bg: Color::from_hex(0x00202e),
            color_block: Color::from_hex(0x003f5c),
            color_open: Color::from_hex(0xfff0d4),
            color_visiting: Color::from_hex(0xbc5090),
            color_visiting_gradient: gradient(Color::from_hex(0xff0000), Color::from_hex(0xbc5090), steps),
            color_path: Color::from_hex(0xff6361),
            color_source: Color::from_hex(0xffa600),
            color_destination: Color::from_hex(0xffa600),
            color_traversed: Color::from_hex(0xcfa093),
            color_perimeter: Color::from_hex(0xc9c982),
        }
    }

    pub fn from(scheme: ColorScheme, steps: usize) -> Self {
        Self {
            color_bg: Color::from_hex(scheme.color_bg),
            color_block: Color::from_hex(scheme.color_block),
            color_open: Color::from_hex(scheme.color_open),
            color_visiting: Color::from_hex(scheme.color_visiting),
            color_visiting_gradient: gradient(
                Color::from_hex(scheme.color_visiting_peak),
                Color::from_hex(scheme.color_visiting),
                steps,
            ),
            color_path: Color::from_hex(scheme.color_path),
            color_source: Color::from_hex(scheme.color_source),
            color_destination: Color::from_hex(scheme.color_destination),
            color_traversed: Color::from_hex(scheme.color_traversed),
            color_perimeter: Color::from_hex(scheme.color_perimeter),
        }
    }

    pub fn shed_color(&self, rank: &Rank) -> &Color {
        self.color_visiting_gradient
            .get((Rank::MAX - rank) as usize)
            .unwrap_or(&self.color_visiting)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ColorScheme {
    pub(crate) color_bg: u32,
    pub(crate) color_block: u32,
    pub(crate) color_open: u32,
    pub(crate) color_visiting: u32,
    pub(crate) color_path: u32,
    pub(crate) color_visiting_peak: u32,
    pub(crate) color_source: u32,
    pub(crate) color_destination: u32,
    pub(crate) color_traversed: u32,
    pub(crate) color_perimeter: u32,
}

impl ColorScheme {
    pub(crate) fn from(path: &Path) -> Self {
        toml::from_str(&fs::read_to_string(path).unwrap()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::maze::tiled::{Maze, UnitShape};

    #[test]
    fn create_context_sets_expected_fields() {
        let context = AmazeingContext::create_context(
            (None, Some(PathBuf::from("tmp/test.maze"))),
            (ArgProcedure::AStar, ArgHeuristic::Octile.heuristic()),
            (3, ArgWeightDirection::Forward.direction()),
            (21, 31),
            (1.25, 48.0, true),
        );

        assert_eq!(context.rows, 21);
        assert_eq!(context.cols, 31);
        assert_eq!(context.jumble_factor, 3);
        assert!(context.show_perimeter);
        assert_eq!(context.context_type, ContextType::Create);
        assert_eq!(context.maze_file_path, Some(PathBuf::from("tmp/test.maze")));
    }

    #[test]
    fn view_and_solve_context_copy_maze_dimensions() {
        let maze = Maze::new_void(UnitShape::Square, 5, 7);

        let view_context = AmazeingContext::view_context(
            (maze.clone(), PathBuf::from("tmp/view.maze")),
            (1.0, 60.0, false),
        );
        assert_eq!(view_context.rows, 5);
        assert_eq!(view_context.cols, 7);
        assert_eq!(view_context.context_type, ContextType::View);

        let solve_context = AmazeingContext::solve_context(
            maze,
            (ArgProcedure::Dfs, ArgHeuristic::Dijkstra.heuristic()),
            (1.0, 60.0, false),
        );
        assert_eq!(solve_context.rows, 5);
        assert_eq!(solve_context.cols, 7);
        assert_eq!(solve_context.context_type, ContextType::Solve);
    }
}

