use crate::maze::{BLOCK, Maze, OPEN, UnitShape};
use crate::util::IsDivisible;
use std::cmp::Ordering;
use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone)]
pub struct NodeFactory {
    pub rows: usize,
    pub cols: usize,
}

impl NodeFactory {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self { rows, cols }
    }

    pub fn at(&self, row: usize, col: usize) -> Option<Node> {
        if row >= self.rows || col >= self.cols {
            None
        } else {
            Some(Node { row, col, rows: self.rows, cols: self.cols })
        }
    }
}

#[derive(Default, Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Hash, Ord)]
pub struct Node {
    pub row: usize,
    pub col: usize,
    rows: usize,
    cols: usize,
}

impl Add<(usize, usize)> for Node {
    type Output = Option<Self>;

    fn add(self, (dr, dc): (usize, usize)) -> Self::Output {
        let row = self.row + dr;
        let col = self.col + dc;
        if row >= self.rows || col >= self.cols {
            None
        } else {
            Some(Node { row, col, rows: self.rows, cols: self.cols })
        }
    }
}

impl Sub<(usize, usize)> for Node {
    type Output = Option<Self>;

    fn sub(self, (dr, dc): (usize, usize)) -> Self::Output {
        if self.row < dr || self.col < dc {
            None
        } else {
            Some(Self { row: self.row - dr, col: self.col - dc, ..self })
        }
    }
}

impl Node {
    #[inline] fn left(self, steps: usize)       -> Option<Self> { self - (0, steps) }
    #[inline] fn right(self, steps: usize)      -> Option<Self> { self + (0, steps) }
    #[inline] fn up(self, steps: usize)         -> Option<Self> { self - (steps, 0) }
    #[inline] fn down(self, steps: usize)       -> Option<Self> { self + (steps, 0) }
    #[inline] fn left_up(self, steps: usize)    -> Option<Self> { self - (steps, steps) }
    #[inline] fn right_down(self, steps: usize) -> Option<Self> { self + (steps, steps) }

    #[inline]
    fn left_down(self, steps: usize) -> Option<Self> {
        (self + (steps, 0))? - (0, steps)
    }

    #[inline]
    fn right_up(self, steps: usize) -> Option<Self> {
        (self - (steps, 0))? + (0, steps)
    }

    pub fn neighbours(self, unit_shape: &UnitShape) -> Vec<Node> {
        let opts: Vec<Option<Node>> = match unit_shape {
            UnitShape::Triangle => match self.row % 4 {
                0 => vec![self.down(1), self.left_down(1), self.up(1)],
                1 => vec![self.right_up(1), self.down(1), self.up(1)],
                2 => vec![self.right_down(1), self.down(1), self.up(1)],
                3 => vec![self.up(1), self.down(1), self.left_up(1)],
                _ => unreachable!(),
            },
            UnitShape::Square | UnitShape::Octagon => {
                vec![self.right(1), self.down(1), self.left(1), self.up(1)]
            }
            UnitShape::Rhombus => {
                if self.row.is_even() {
                    vec![self.down(1), self.left_down(1), self.left_up(1), self.up(1)]
                } else {
                    vec![self.right_down(1), self.down(1), self.up(1), self.right_up(1)]
                }
            }
            UnitShape::Hexagon => {
                if self.row.is_even() {
                    vec![self.right(1), self.down(1), self.left_down(1), self.left(1), self.left_up(1), self.up(1)]
                } else {
                    vec![self.right(1), self.right_down(1), self.down(1), self.left(1), self.up(1), self.right_up(1)]
                }
            }
            UnitShape::OctagonSquare => {
                if self.row.is_even() {
                    vec![
                        self.right(1),
                        self.down(1),
                        self.down(2),
                        self.left_down(1),
                        self.left(1),
                        self.left_up(1),
                        self.up(2),
                        self.up(1),
                    ]
                } else {
                    vec![self.right_down(1), self.down(1), self.up(1), self.right_up(1)]
                }
            }
            UnitShape::HexagonRectangle => {
                if self.row.is_even() {
                    vec![self.right(1), self.down(1), self.left_down(1), self.left(1), self.left_up(1), self.up(1)]
                } else {
                    vec![self.right_down(1), self.down(1), self.up(1), self.right_up(1)]
                }
            }
        };
        opts.into_iter().flatten().collect()
    }

    pub fn neighbours_open(self, maze: &Maze, unit_shape: &UnitShape) -> Vec<Node> {
        self.neighbours(unit_shape).into_iter().filter(|p| maze[*p] == OPEN).collect()
    }

    pub fn neighbours_block(self, maze: &Maze, unit_shape: &UnitShape) -> Vec<Node> {
        self.neighbours(unit_shape).into_iter().filter(|p| maze[*p] == BLOCK).collect()
    }
}

pub trait DNodeWeighted: Ord {
    fn new(node: Node, cost: u32, heu_cost: u32) -> Self;
    fn node(&self) -> Node;
    fn cost(&self) -> u32;
    fn heu_cost(&self) -> u32;
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DNodeWeightedForward {
    pub(crate) node: Node,
    pub(crate) cost: u32,
    pub(crate) heu_cost: u32,
}

impl PartialOrd for DNodeWeightedForward {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DNodeWeightedForward {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heu_cost.cmp(&self.heu_cost)
    }
}

impl DNodeWeighted for DNodeWeightedForward {
    fn new(node: Node, cost: u32, heu_cost: u32) -> Self {
        Self { node, cost, heu_cost }
    }
    fn node(&self) -> Node { self.node }
    fn cost(&self) -> u32 { self.cost }
    fn heu_cost(&self) -> u32 { self.heu_cost }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DNodeWeightedBackward {
    pub(crate) node: Node,
    pub(crate) cost: u32,
    pub(crate) heu_cost: u32,
}

impl PartialOrd for DNodeWeightedBackward {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DNodeWeightedBackward {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heu_cost.cmp(&self.heu_cost).reverse()
    }
}

impl DNodeWeighted for DNodeWeightedBackward {
    fn new(node: Node, cost: u32, heu_cost: u32) -> Self {
        Self { node, cost, heu_cost }
    }
    fn node(&self) -> Node { self.node }
    fn cost(&self) -> u32 { self.cost }
    fn heu_cost(&self) -> u32 { self.heu_cost }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum WeightDirection {
    Forward,
    Backward,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::maze::{BLOCK, Maze, UnitShape};
    use std::collections::BinaryHeap;

    #[test]
    fn node_factory_bounds_are_enforced() {
        let f = NodeFactory::new(2, 3);
        assert!(f.at(1, 2).is_some());
        assert!(f.at(2, 0).is_none());
        assert!(f.at(0, 3).is_none());
    }

    #[test]
    fn node_add_and_sub_respect_grid_bounds() {
        let f = NodeFactory::new(3, 3);
        let n = f.at(1, 1).unwrap();
        assert_eq!((n + (1, 1)).unwrap(), f.at(2, 2).unwrap());
        assert!((n + (2, 0)).is_none());
        assert_eq!((n - (1, 1)).unwrap(), f.at(0, 0).unwrap());
        assert!((n - (2, 0)).is_none());
    }

    #[test]
    fn neighbours_cover_shape_specific_rules() {
        let f = NodeFactory::new(5, 5);
        let center_even = f.at(2, 2).unwrap();
        let center_odd = f.at(3, 2).unwrap();

        assert_eq!(center_even.neighbours(&UnitShape::Square).len(), 4);
        assert_eq!(center_even.neighbours(&UnitShape::Octagon).len(), 4);
        assert_eq!(center_even.neighbours(&UnitShape::Rhombus).len(), 4);
        assert_eq!(center_odd.neighbours(&UnitShape::Rhombus).len(), 4);
        assert_eq!(center_even.neighbours(&UnitShape::Hexagon).len(), 6);
        assert_eq!(center_odd.neighbours(&UnitShape::Hexagon).len(), 6);
        assert_eq!(center_even.neighbours(&UnitShape::HexagonRectangle).len(), 6);
        assert_eq!(center_odd.neighbours(&UnitShape::HexagonRectangle).len(), 4);
        assert_eq!(center_even.neighbours(&UnitShape::OctagonSquare).len(), 8);
        assert_eq!(center_odd.neighbours(&UnitShape::OctagonSquare).len(), 4);

        assert_eq!(
            center_even.neighbours(&UnitShape::Rhombus),
            vec![f.at(3, 2).unwrap(), f.at(3, 1).unwrap(), f.at(1, 1).unwrap(), f.at(1, 2).unwrap()]
        );
        assert_eq!(
            center_odd.neighbours(&UnitShape::Rhombus),
            vec![f.at(4, 3).unwrap(), f.at(4, 2).unwrap(), f.at(2, 2).unwrap(), f.at(2, 3).unwrap()]
        );
    }

    #[test]
    fn neighbours_open_and_block_filter_cells() {
        let mut maze = Maze::new(UnitShape::Square, 3, 3, BLOCK);
        let f = NodeFactory::new(3, 3);
        let c = f.at(1, 1).unwrap();
        maze[f.at(1, 2).unwrap()] = OPEN;
        maze[f.at(0, 1).unwrap()] = OPEN;

        let open = c.neighbours_open(&maze, &UnitShape::Square);
        let block = c.neighbours_block(&maze, &UnitShape::Square);

        assert_eq!(open.len(), 2);
        assert_eq!(block.len(), 2);
    }

    #[test]
    fn weighted_node_ordering_works_for_heap() {
        let f = NodeFactory::new(2, 2);
        let a = f.at(0, 0).unwrap();
        let b = f.at(0, 1).unwrap();

        let mut forward = BinaryHeap::new();
        forward.push(DNodeWeightedForward::new(a, 0, 1));
        forward.push(DNodeWeightedForward::new(b, 0, 5));
        assert_eq!(forward.pop().unwrap().heu_cost(), 1);

        let mut backward = BinaryHeap::new();
        backward.push(DNodeWeightedBackward::new(a, 0, 1));
        backward.push(DNodeWeightedBackward::new(b, 0, 5));
        assert_eq!(backward.pop().unwrap().heu_cost(), 5);
    }
}
