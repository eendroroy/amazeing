use crate::_lib::tiled::{Maze, NodeHeuFn};
use crate::command::ArgSolveProcedure;

#[derive(Debug, Clone)]
pub struct SolveContext {
    pub(crate) maze: Maze,
    pub(crate) procedure: ArgSolveProcedure,
    pub(crate) heuristic: NodeHeuFn,
}
