use crate::command::ArgGenProcedure;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct CreateContext {
    pub(crate) maze_file_path: Option<PathBuf>,
    pub(crate) procedure: ArgGenProcedure,
    pub(crate) rows: usize,
    pub(crate) cols: usize,
}
