use crate::square::maze::Maze;
use std::ops::{Index, IndexMut};

impl<const ROWS: usize, const COLS: usize> Index<(usize, usize)> for Maze<ROWS, COLS> {
    type Output = u8;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0][index.1]
    }
}

impl<const ROWS: usize, const COLS: usize> IndexMut<(usize, usize)> for Maze<ROWS, COLS> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0][index.1]
    }
}
