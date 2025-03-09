use amazeing::maze::matrix::Maze;

#[derive(Debug, Clone)]
pub struct SolverContext {
    pub(crate) source: (usize, usize),
    pub(crate) destination: (usize, usize),
    pub(crate) title: String,
    pub(crate) maze: Maze,
    pub(crate) fps: u8,
    pub(crate) tracer: Vec<Vec<(usize, usize)>>,
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
