use crate::command::ArgGenProcedure;
use amazeing::matrix::Node;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct CreateContext {
    pub(crate) maze_file_path: Option<PathBuf>,
    pub(crate) sources: Vec<Node>,
    pub(crate) procedure: ArgGenProcedure,
    pub(crate) rows: usize,
    pub(crate) cols: usize,
}
