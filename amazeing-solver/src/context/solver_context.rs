use amazeing::solver::matrix::dijkstra_heuristic;

#[derive(Debug, Clone)]
pub struct SolverContext {
    pub(crate) maze_data: Vec<Vec<u32>>,
    pub(crate) from: (usize, usize),
    pub(crate) to: (usize, usize),
    pub(crate) heuristic: fn((usize, usize), (usize, usize)) -> u32,
    pub(crate) fps: u8,
}

impl SolverContext {
    pub fn new() -> Self {
        Self {
            fps: 7u8,
            maze_data: vec![],
            from: (0, 0),
            to: (0, 0),
            heuristic: dijkstra_heuristic,
        }
    }
}
