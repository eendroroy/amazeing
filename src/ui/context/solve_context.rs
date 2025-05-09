use crate::command::ArgSolveProcedure;
use crate::core::tiled::{Maze, NodeHeuFn};

#[derive(Debug, Clone)]
pub struct SolveContext {
    pub(crate) maze: Maze,
    pub(crate) procedure: ArgSolveProcedure,
    pub(crate) heuristic: NodeHeuFn,
}
