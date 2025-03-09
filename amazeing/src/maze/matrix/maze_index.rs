use crate::maze::matrix::Maze;
use crate::structure::DNode;
use std::ops::{Index, IndexMut};

impl Index<DNode> for Maze {
    type Output = u32;

    fn index(&self, index: DNode) -> &Self::Output {
        &self.data[index.0][index.1]
    }
}

impl IndexMut<DNode> for Maze {
    fn index_mut(&mut self, index: DNode) -> &mut Self::Output {
        &mut self.data[index.0][index.1]
    }
}
