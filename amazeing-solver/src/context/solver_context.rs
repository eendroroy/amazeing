use amazeing::solver::matrix::Maze;

#[derive(Debug, Clone)]
pub struct SolverContext {
    pub(crate) title: String,
    pub(crate) maze: Maze,
    pub(crate) fps: u8,
    pub(crate) tracer: Vec<Vec<(usize, usize)>>,
}

impl SolverContext {
    pub fn new() -> Self {
        Self {
            title: "Solver".to_string(),
            fps: 7u8,
            maze: Maze::new(),
            tracer: vec![vec![]],
        }
    }
}
