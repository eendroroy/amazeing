use crate::command::ArgGenProcedure;
use amazeing::Node;
use std::path::PathBuf;
use std::sync::{LazyLock, RwLock};

type Ctx = LazyLock<RwLock<GenerationContext>>;

pub struct GenerationContext {
    pub(crate) maze_file_path: Option<PathBuf>,
    pub(crate) source: Node,
    pub(crate) procedure: ArgGenProcedure,
    pub(crate) rows: usize,
    pub(crate) cols: usize,
    pub(crate) fps: u8,
}

impl GenerationContext {
    pub fn new() -> Self {
        Self {
            maze_file_path: None,
            source: Node::default(),
            procedure: ArgGenProcedure::Bfs,
            rows: 10,
            cols: 10,
            fps: 10,
        }
    }
}

pub static GEN_CTX: Ctx = LazyLock::new(|| RwLock::new(GenerationContext::new()));
