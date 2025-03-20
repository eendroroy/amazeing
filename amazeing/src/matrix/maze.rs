use crate::matrix::types::Node;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Maze {
    pub(crate) data: Vec<Vec<u32>>,
}

impl Maze {
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn from(data: Vec<Vec<u32>>) -> Self {
        Self { data }
    }

    pub fn set_data(&mut self, data: Vec<Vec<u32>>) {
        self.data = data
    }

    pub fn rows(&self) -> usize {
        self.data.iter().len()
    }

    pub fn cols(&self) -> usize {
        self.data.get(0).unwrap().iter().len()
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
