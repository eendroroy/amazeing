use amazeing::maze::matrix::Maze;
use amazeing::DNode;

#[derive(Debug, Clone)]
pub struct SolverContext {
    pub(crate) source: DNode,
    pub(crate) destination: DNode,
    pub(crate) title: String,
    pub(crate) maze: Maze,
    pub(crate) fps: u8,
    pub(crate) tracer: Vec<Vec<DNode>>,
}

impl SolverContext {
    pub fn new() -> Self {
        Self {
            source: (0, 0),
            destination: (0, 0),
            title: "Solver".to_string(),
            fps: 7u8,
            maze: Maze::new(),
            tracer: vec![vec![]],
        }
    }
}
