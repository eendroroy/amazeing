pub type DNode = (usize, usize);
pub type HeuFn = fn(DNode, DNode) -> u32;

pub(crate) type FnNeighbour = fn(DNode) -> Option<DNode>;
