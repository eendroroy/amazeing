pub type Node = (usize, usize);
pub type NodeHeuFn = fn(Node, Node) -> u32;
pub type Tracer = Vec<Vec<Node>>;

pub(crate) type NeighbourFn = fn(Node) -> Option<Node>;
