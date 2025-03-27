use std::collections::HashMap;

/// A `Node` represents a position in a matrix, defined by its row and column indices.
pub type Node = (usize, usize);

/// A `NodeHeuFn` is a function that calculates the heuristic cost between two nodes.
pub type NodeHeuFn = fn(Node, Node) -> u32;

/// A `Tracer` is a 2D vector of nodes, used to trace paths in the matrix.
pub type Tracer = Vec<HashMap<Node, bool>>;

pub(crate) type NeighbourFn = fn(Node) -> Option<Node>;
