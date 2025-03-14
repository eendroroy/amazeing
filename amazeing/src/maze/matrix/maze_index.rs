use crate::maze::matrix::Maze;
use crate::Node;
use std::ops::{Index, IndexMut};

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
