use crate::command::ArgSolveProcedure;
use amazeing::matrix::{Maze, NodeHeuFn};

#[derive(Debug, Clone)]
pub struct SolveContext {
    pub(crate) maze: Maze,
    pub(crate) maze_file_path: String,
    pub(crate) procedure: ArgSolveProcedure,
    pub(crate) heuristic: NodeHeuFn,
    pub(crate) tempo: u8,
}
