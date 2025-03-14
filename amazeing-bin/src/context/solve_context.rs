use amazeing::maze::matrix::{dijkstra_heuristic, Maze};
use amazeing::NodeHeuFn;
use std::path::PathBuf;
use std::sync::{LazyLock, RwLock};
use crate::command::ArgSolveProcedure;

type Ctx = LazyLock<RwLock<SolveContext>>;

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
