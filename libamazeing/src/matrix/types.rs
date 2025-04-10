use std::collections::{HashMap, VecDeque};

/// A `Node` represents a position in a matrix, defined by its row and column indices.
pub type Node = (usize, usize);

/// A `NodeHeuFn` is a function that calculates the heuristic cost between two nodes.
pub type NodeHeuFn = fn(Node, Node) -> u32;

pub type Rank = i32;
/// A `Trace` is a mapping of `Node` to its `Rank`, used to store rankings or scores for nodes.
pub type Trace = HashMap<Node, Rank>;
/// A `Tracer` is a 2D vector of nodes, used to trace paths in the matrix.
pub type Tracer = Vec<Trace>;

pub(crate) type NeighbourFn = fn(Node) -> Option<Node>;

pub(crate) type Push = fn(&mut VecDeque<Node>, Node);
pub(crate) type Pop = fn(&mut VecDeque<Node>) -> Option<Node>;
