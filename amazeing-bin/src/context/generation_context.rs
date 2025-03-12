use crate::cli::ArgGenProcedure;
use std::path::PathBuf;
use std::sync::{LazyLock, RwLock};

type Ctx = LazyLock<RwLock<GenerationContext>>;

pub struct GenerationContext {
    pub(crate) maze_file_path: PathBuf,
    pub(crate) procedure: ArgGenProcedure,
    pub(crate) rows: usize,
    pub(crate) cols: usize,
}

impl GenerationContext {
    pub fn new() -> Self {
        Self {
            maze_file_path: PathBuf::new(),
            procedure: ArgGenProcedure::Bfs,
            rows: 10,
            cols: 10,
        }
    }
}

pub static GEN_CTX: Ctx = LazyLock::new(|| RwLock::new(GenerationContext::new()));
