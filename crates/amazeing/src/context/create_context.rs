use crate::command::ArgGenProcedure;
use amazeing::tiled::NodeHeuFn;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct CreateContext {
    pub(crate) maze_file_path: Option<PathBuf>,
    pub(crate) procedure: ArgGenProcedure,
    pub(crate) heuristic: NodeHeuFn,
    pub(crate) jumble_factor: u32,
    pub(crate) rows: usize,
    pub(crate) cols: usize,
}

impl CreateContext {
    pub fn from(
        maze_file_path: Option<PathBuf>,
        procedure: ArgGenProcedure,
        heuristic: NodeHeuFn,
        jumble_factor: u32,
        rows: usize,
        cols: usize,
    ) -> Self {
        Self {
            maze_file_path,
            procedure,
            heuristic,
            jumble_factor,
            rows,
            cols,
        }
    }
}
