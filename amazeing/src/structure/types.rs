pub type DNode = (usize, usize);
pub type Heu = fn(DNode, DNode) -> u32;

pub(crate) type FnNeighbour = fn(DNode) -> Option<DNode>;
