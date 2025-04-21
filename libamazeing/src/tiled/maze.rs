use super::types::Node;
use crate::tiled::{MazeShape, UnitShape};
use std::ops::{Index, IndexMut};

#[derive(Default, Debug, Clone)]
pub struct Maze {
    pub maze_shape: MazeShape,
    pub unit_shape: UnitShape,
    pub data: Vec<Vec<u32>>,
}

impl Maze {
    pub fn from(maze_shape: MazeShape, unit_shape: UnitShape, data: Vec<Vec<u32>>) -> Self {
        Self {
            maze_shape,
            unit_shape,
            data,
        }
    }

    pub fn rows(&self) -> usize {
        self.data.iter().len()
    }

    pub fn cols(&self) -> usize {
        self.data.first().unwrap().iter().len()
    }
}

impl Index<Node> for Maze {
    type Output = u32;

    fn index(&self, index: Node) -> &Self::Output {
        &self.data[index.0][index.1]
    }
}

impl IndexMut<Node> for Maze {
    fn index_mut(&mut self, index: Node) -> &mut Self::Output {
        &mut self.data[index.0][index.1]
    }
}
