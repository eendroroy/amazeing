use crate::core::tiled::node::Node;
use crate::core::tiled::{MazeData, MazeShape, UnitShape, VOID};
use std::ops::{Index, IndexMut};

#[derive(Default, Debug, Clone)]
pub struct Maze {
    pub maze_shape: MazeShape,
    pub unit_shape: UnitShape,
    pub data: MazeData,
}

impl Maze {
    pub fn new(maze_shape: MazeShape, unit_shape: UnitShape, rows: usize, cols: usize, default: i8) -> Self {
        Self {
            maze_shape,
            unit_shape,
            data: vec![vec![default; cols]; rows],
        }
    }
    pub fn new_void(maze_shape: MazeShape, unit_shape: UnitShape, rows: usize, cols: usize) -> Self {
        Maze::new(maze_shape, unit_shape, rows, cols, VOID)
    }

    pub fn from(maze_shape: MazeShape, unit_shape: UnitShape, data: MazeData) -> Self {
        Self { maze_shape, unit_shape, data }
    }

    pub fn rows(&self) -> usize {
        self.data.iter().len()
    }

    pub fn cols(&self) -> usize {
        self.data.first().unwrap().iter().len()
    }
}

impl Index<Node> for Maze {
    type Output = i8;

    fn index(&self, index: Node) -> &Self::Output {
        &self.data[index.row][index.col]
    }
}

impl IndexMut<Node> for Maze {
    fn index_mut(&mut self, index: Node) -> &mut Self::Output {
        &mut self.data[index.row][index.col]
    }
}
