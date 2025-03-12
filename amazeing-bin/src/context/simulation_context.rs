use crate::cli::ArgSolveProcedure;
use amazeing::maze::matrix::{dijkstra_heuristic, Maze};
use amazeing::{DNode, HeuFn};
use std::path::PathBuf;
use std::sync::{LazyLock, RwLock};

type Ctx = LazyLock<RwLock<SimulationContext>>;

pub struct SimulationContext {
    pub(crate) maze_file_path: PathBuf,
    pub(crate) maze: Maze,
    pub(crate) source: DNode,
    pub(crate) destination: DNode,
    pub(crate) proc: ArgSolveProcedure,
    pub(crate) heuristic: HeuFn,
    pub(crate) fps: u8,
    pub(crate) tracer: Vec<Vec<DNode>>,
}

impl SimulationContext {
    pub fn new() -> Self {
        Self {
            maze_file_path: PathBuf::new(),
            maze: Maze::new(),
            source: (usize::MIN, usize::MIN),
            destination: (usize::MIN, usize::MIN),
            proc: ArgSolveProcedure::Bfs,
            heuristic: dijkstra_heuristic,
            fps: 5,
            tracer: vec![],
        }
    }
}

pub static SIM_CTX: Ctx = LazyLock::new(|| RwLock::new(SimulationContext::new()));
