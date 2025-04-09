use std::collections::HashMap;

/// A `Node` represents a position in a matrix, defined by its row and column indices.
pub type Node = (usize, usize);

/// A `NodeHeuFn` is a function that calculates the heuristic cost between two nodes.
pub type NodeHeuFn = fn(Node, Node) -> u32;

/// A `Tracer` is a 2D vector of nodes, used to trace paths in the matrix.
pub type Tracer = Vec<HashMap<Node, bool>>;

pub(crate) type NeighbourFn = fn(Node) -> Option<Node>;

/// `NavMode` defines the navigation modes for traversing a matrix.
/// Each variant represents a specific type of movement:
/// - `SquareSide`: Movement along the four sides of a square.
/// - `SquareAdjacent`: Movement to all eight adjacent positions in a square grid.
/// - `HexagonalSide`: Movement along the six sides of a hexagon.
/// - `HexagonalAdjacent`: Movement to all twelve adjacent positions in a hexagonal grid.
pub enum NavMode {
    Square,
    Hexagonal,
}

impl NavMode {
    /// Returns the number of sides or adjacent positions based on the navigation mode.
    pub fn sides(&self) -> usize {
        match self {
            NavMode::Square => 4,
            NavMode::Hexagonal => 6,
        }
    }
}
