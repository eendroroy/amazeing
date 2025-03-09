use crate::context::common::display_size;
use amazeing::maze::matrix::Maze;
use amazeing::DNode;

#[derive(Debug, Clone)]
pub struct SolverContext {
    pub(crate) maze: Maze,
    pub(crate) source: DNode,
    pub(crate) destination: DNode,
    pub(crate) title: String,
    pub(crate) fps: u8,
    pub(crate) display_size: String,
    pub(crate) tracer: Vec<Vec<DNode>>,
}

impl SolverContext {
    pub fn new() -> Self {
        Self {
            maze: Maze::new(),
            source: (0, 0),
            destination: (0, 0),
            title: "Solver".to_string(),
            fps: 7u8,
            display_size: String::from("medium"),
            tracer: vec![vec![]],
        }
    }

    pub fn display_size(&self) -> (f32, f32, f32, f32) {
        display_size(&*self.display_size)
    }
}
