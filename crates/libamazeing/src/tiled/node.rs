use crate::tiled::{BLOCK, Maze, OPEN, UnitShape};
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

    pub fn at(&self, row: usize, col: usize) -> Self {
        Self { row, col, ..*self }
    }

    pub fn center(&self) -> Self {
        Self {
            row: self.row / 2,
            col: self.col / 2,
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

    pub fn surroundings(self) -> Vec<Node> {
        [
            Node::right,
            Node::right_down,
            Node::right_up,
            Node::left,
            Node::left_down,
            Node::left_up,
            Node::down,
            Node::up,
        ]
        .iter()
        .filter_map(|i| i(self))
        .collect()
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
            UnitShape::Square => vec![Node::right, Node::down, Node::left, Node::up],
            UnitShape::Hexagon | UnitShape::Circle => {
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
