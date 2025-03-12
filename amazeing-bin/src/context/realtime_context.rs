use crate::cli::ArgSolveProcedure;
use amazeing::maze::matrix::{dijkstra_heuristic, Maze};
use amazeing::HeuFn;
use std::path::PathBuf;
use std::sync::{LazyLock, RwLock};

type Ctx = LazyLock<RwLock<RealtimeContext>>;

pub struct RealtimeContext {
    pub(crate) maze_file_path: PathBuf,
    pub(crate) maze: Maze,
    pub(crate) proc: ArgSolveProcedure,
    pub(crate) heuristic: HeuFn,
}

impl RealtimeContext {
    pub fn new() -> Self {
        Self {
            maze_file_path: PathBuf::new(),
            maze: Maze::new(),
            proc: ArgSolveProcedure::Bfs,
            heuristic: dijkstra_heuristic,
        }
    }
}

pub static REL_CTX: Ctx = LazyLock::new(|| RwLock::new(RealtimeContext::new()));
