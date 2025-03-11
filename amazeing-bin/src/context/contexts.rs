use crate::command::{Heuristic, Proc};
use crate::context::common::display_size;
use crate::context::{Colors, DrawContext};
use amazeing::maze::matrix::Maze;
use amazeing::DNode;
use std::sync::{LazyLock, RwLock};

type Ctx = LazyLock<RwLock<AmazeingContext>>;

pub struct AmazeingContext {
    pub(crate) colors: Colors,
    pub(crate) maze_file_path: String,
    pub(crate) maze: Maze,
    pub(crate) source: DNode,
    pub(crate) destination: DNode,
    pub(crate) proc: Proc,
    pub(crate) heuristic: Heuristic,
    pub(crate) tracer: Vec<Vec<DNode>>,
    pub(crate) fps: u8,
    pub(crate) rows: usize,
    pub(crate) cols: usize,
    pub(crate) display_size: String,
}

impl AmazeingContext {
    pub fn new() -> Self {
        Self {
            colors: Colors::new(),
            maze_file_path: String::new(),
            maze: Maze::new(),
            source: (usize::MIN, usize::MIN),
            destination: (usize::MIN, usize::MIN),
            proc: Proc::None,
            heuristic: Heuristic::Dijkstra,
            tracer: vec![],
            fps: 5,
            rows: 10,
            cols: 10,
            display_size: String::new(),
        }
    }

    pub fn display_size(&self) -> (f32, f32, f32, f32) {
        display_size(&*self.display_size)
    }

    pub fn screen_size(&self) -> (f32, f32) {
        let dc = self.draw_context();
        (
            dc.margin + self.maze.cols() as f32 * (dc.cell_width + dc.padding) + dc.margin,
            dc.margin + self.maze.rows() as f32 * (dc.cell_width + dc.padding) + dc.margin,
        )
    }

    pub fn draw_context(&self) -> DrawContext {
        let size = self.display_size();
        DrawContext {
            margin: size.0,
            padding: size.1,
            cell_width: size.2,
            cell_height: size.3,
        }
    }
}

pub static CONTEXT: Ctx = LazyLock::new(|| RwLock::new(AmazeingContext::new()));
