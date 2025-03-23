use crate::command::ArgSolveProcedure;
use amazeing::matrix::{heuristics::dijkstra_heuristic, Maze, NodeHeuFn};
use std::path::PathBuf;
use std::sync::{LazyLock, RwLock};

type Ctx = LazyLock<RwLock<SolveContext>>;

#[derive(Debug, Clone)]
pub struct SolveContext {
    pub(crate) maze_file_path: PathBuf,
    pub(crate) maze: Maze,
    pub(crate) proc: ArgSolveProcedure,
    pub(crate) heuristic: NodeHeuFn,
    pub(crate) fps: u8,
}

impl SolveContext {
    pub fn new() -> Self {
        Self {
            maze_file_path: PathBuf::new(),
            maze: Maze::new(),
            proc: ArgSolveProcedure::Bfs,
            heuristic: dijkstra_heuristic,
            fps: 5,
        }
    }
}

pub static SOLVE_CTX: Ctx = LazyLock::new(|| RwLock::new(SolveContext::new()));
