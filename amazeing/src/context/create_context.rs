use crate::command::ArgGenProcedure;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct CreateContext {
    pub(crate) maze_file_path: Option<PathBuf>,
    pub(crate) procedure: ArgGenProcedure,
    pub(crate) rows: usize,
    pub(crate) cols: usize,
}

impl CreateContext {
    pub fn from(maze_file_path: Option<PathBuf>, procedure: ArgGenProcedure, rows: usize, cols: usize) -> Self {
        Self {
            maze_file_path,
            procedure,
            rows,
            cols,
        }
    }
}
