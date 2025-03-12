use amazeing::maze::matrix::Maze;
use std::path::PathBuf;
use std::sync::{LazyLock, RwLock};

type Ctx = LazyLock<RwLock<ModifyContext>>;

pub struct ModifyContext {
    pub(crate) maze_file_path: PathBuf,
    pub(crate) maze: Maze,
}

impl ModifyContext {
    pub fn new() -> Self {
        Self {
            maze_file_path: PathBuf::new(),
            maze: Maze::new(),
        }
    }
}

pub static MOD_CTX: Ctx = LazyLock::new(|| RwLock::new(ModifyContext::new()));
