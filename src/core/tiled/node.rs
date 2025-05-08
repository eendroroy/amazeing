use crate::core::tiled::{BLOCK, Maze, OPEN, UnitShape};
use std::cmp::Ordering;
use std::ops::{Add, Sub};

#[derive(Default, Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Hash, Ord)]
pub struct Node {
    pub row: usize,
    pub col: usize,
    pub rows: usize,
    pub cols: usize,
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
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            ..Default::default()
        }
    }

    pub fn at(&self, row: usize, col: usize) -> Option<Self> {
        if row >= self.rows || col >= self.cols {
            None
        } else {
            Some(Self { row, col, ..*self })
        }
    }

    pub fn center(&self) -> Self {
        Self {
            row: self.rows / 2,
            col: self.cols / 2,
            ..*self
        }
    }

    pub fn left(self) -> Option<Self> {
        self - (0, 1)
    }

    pub fn right(self) -> Option<Self> {
        self + (0, 1)
    }

    pub fn up(self) -> Option<Self> {
        self - (1, 0)
    }

    pub fn down(self) -> Option<Self> {
        self + (1, 0)
    }

    pub fn left_up(self) -> Option<Self> {
        self - (1, 1)
    }

    pub fn left_down(self) -> Option<Self> {
        if let Some(data) = self + (1, 0) { data - (0, 1) } else { None }
    }

    pub fn right_up(self) -> Option<Self> {
        if let Some(data) = self - (1, 0) { data + (0, 1) } else { None }
    }

    pub fn right_down(self) -> Option<Self> {
        self + (1, 1)
    }

    pub fn neighbours(self, unit_shape: &UnitShape) -> Vec<Node> {
        match unit_shape {
            UnitShape::Triangle => match self.row % 4 {
                0 => vec![Node::down, Node::left_down, Node::up],
                1 => vec![Node::right_up, Node::down, Node::up],
                2 => vec![Node::right_down, Node::down, Node::up],
                3 => vec![Node::up, Node::down, Node::left_up],
                _ => unreachable!(),
            },
            UnitShape::Square | UnitShape::Octagon => vec![Node::right, Node::down, Node::left, Node::up],
            UnitShape::Hexagon => {
                if self.row % 2 == 0 {
                    vec![
                        Node::right,
                        Node::down,
                        Node::left_down,
                        Node::left,
                        Node::left_up,
                        Node::up,
                    ]
                } else {
                    vec![
                        Node::right,
                        Node::right_down,
                        Node::down,
                        Node::left,
                        Node::up,
                        Node::right_up,
                    ]
                }
            }
        }
        .iter()
        .filter_map(|i| i(self))
        .collect()
    }

    pub fn neighbours_open(self, maze: &Maze, unit_shape: &UnitShape) -> Vec<Node> {
        self.neighbours(unit_shape)
            .into_iter()
            .filter(|p| maze[*p] == OPEN)
            .collect()
    }

    pub fn neighbours_block(self, maze: &Maze, unit_shape: &UnitShape) -> Vec<Node> {
        self.neighbours(unit_shape)
            .into_iter()
            .filter(|p| maze[*p] == BLOCK)
            .collect()
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
