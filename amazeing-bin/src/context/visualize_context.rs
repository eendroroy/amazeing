use amazeing::matrix::Maze;
use std::path::PathBuf;
use std::sync::{LazyLock, RwLock};

type Ctx = LazyLock<RwLock<VisualizeContext>>;

#[derive(Debug, Clone)]
pub struct VisualizeContext {
    pub(crate) maze_file_path: PathBuf,
    pub(crate) maze: Maze,
}

impl VisualizeContext {
    pub fn new() -> Self {
        Self {
            maze_file_path: PathBuf::new(),
            maze: Maze::new(),
        }
    }
}

pub static VIS_CTX: Ctx = LazyLock::new(|| RwLock::new(VisualizeContext::new()));
