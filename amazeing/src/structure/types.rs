pub type DNode = (usize, usize);
pub(crate) type FnNeighbour = fn(DNode) -> Option<DNode>;
