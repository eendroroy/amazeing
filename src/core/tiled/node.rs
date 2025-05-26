use crate::core::tiled::{BLOCK, Maze, OPEN, UnitShape};
use crate::utility::IsDivisible;
use std::cmp::Ordering;
use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone)]
pub(crate) struct NodeFactory {
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
            Some(Node {
                row,
                col,
                rows: self.rows,
                cols: self.cols,
            })
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

    fn add(self, rhs: (usize, usize)) -> Self::Output {
        let row = self.row + rhs.0;
        let col = self.col + rhs.1;

        if row >= self.rows || col >= self.cols {
            None
        } else {
            Some(Node {
                row,
                col,
                rows: self.rows,
                cols: self.cols,
            })
        }
    }
}

impl Sub<(usize, usize)> for Node {
    type Output = Option<Self>;

    fn sub(self, rhs: (usize, usize)) -> Self::Output {
        if self.row < rhs.0 || self.col < rhs.1 {
            None
        } else {
            Some(Self {
                row: self.row - rhs.0,
                col: self.col - rhs.1,
                ..self
            })
        }
    }
}

impl Node {
    pub fn left(self, steps: usize) -> Box<dyn Fn(Node) -> Option<Node>> {
        Box::new(move |n| n - (0, steps))
    }

    pub fn right(self, steps: usize) -> Box<dyn Fn(Node) -> Option<Node>> {
        Box::new(move |n| n + (0, steps))
    }

    pub fn up(self, steps: usize) -> Box<dyn Fn(Node) -> Option<Node>> {
        Box::new(move |n| n - (steps, 0))
    }

    pub fn down(self, steps: usize) -> Box<dyn Fn(Node) -> Option<Node>> {
        Box::new(move |n| n + (steps, 0))
    }

    pub fn left_up(self, steps: usize) -> Box<dyn Fn(Node) -> Option<Node>> {
        Box::new(move |n| n - (steps, steps))
    }

    pub fn left_down(self, steps: usize) -> Box<dyn Fn(Node) -> Option<Node>> {
        Box::new(move |n| if let Some(data) = n + (steps, 0) { data - (0, steps) } else { None })
    }

    pub fn right_up(self, steps: usize) -> Box<dyn Fn(Node) -> Option<Node>> {
        Box::new(move |n| if let Some(data) = n - (steps, 0) { data + (0, steps) } else { None })
    }

    pub fn right_down(self, steps: usize) -> Box<dyn Fn(Node) -> Option<Node>> {
        Box::new(move |n| n + (steps, steps))
    }

    pub fn neighbours(self, unit_shape: &UnitShape) -> Vec<Node> {
        match unit_shape {
            UnitShape::Triangle => match self.row % 4 {
                0 => vec![self.down(1), self.left_down(1), self.up(1)],
                1 => vec![self.right_up(1), self.down(1), self.up(1)],
                2 => vec![self.right_down(1), self.down(1), self.up(1)],
                3 => vec![self.up(1), self.down(1), self.left_up(1)],
                _ => unreachable!(),
            },
            UnitShape::Square | UnitShape::Octagon => vec![self.right(1), self.down(1), self.left(1), self.up(1)],
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
        }
        .iter()
        .filter_map(|i| i(self))
        .collect()
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

impl PartialOrd<Self> for DNodeWeightedForward {
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

    fn node(&self) -> Node {
        self.node
    }

    fn cost(&self) -> u32 {
        self.cost
    }

    fn heu_cost(&self) -> u32 {
        self.heu_cost
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DNodeWeightedBackward {
    pub(crate) node: Node,
    pub(crate) cost: u32,
    pub(crate) heu_cost: u32,
}

impl PartialOrd<Self> for DNodeWeightedBackward {
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

    fn node(&self) -> Node {
        self.node
    }

    fn cost(&self) -> u32 {
        self.cost
    }

    fn heu_cost(&self) -> u32 {
        self.heu_cost
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum WeightDirection {
    Forward,
    Backward,
}
