use crate::solver::matrix::Maze;
use std::ops::{Index, IndexMut};
use crate::solver::matrix::neighbour::DNode;

impl<const ROWS: usize, const COLS: usize> Index<DNode> for Maze<ROWS, COLS> {
    type Output = u8;

    fn index(&self, index: DNode) -> &Self::Output {
        &self.data[index.0][index.1]
    }
}

impl<const ROWS: usize, const COLS: usize> IndexMut<DNode> for Maze<ROWS, COLS> {
    fn index_mut(&mut self, index: DNode) -> &mut Self::Output {
        &mut self.data[index.0][index.1]
    }
}
