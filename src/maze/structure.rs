use crate::maze::node::Node;
use crate::maze::{MazeData, UnitShape, VOID};
use std::ops::{Index, IndexMut};

#[derive(Default, Debug, Clone)]
pub struct Maze {
    pub unit_shape: UnitShape,
    pub data: MazeData,
}

impl Maze {
    pub fn new(unit_shape: UnitShape, rows: usize, cols: usize, default: i8) -> Self {
        Self {
            unit_shape,
            data: vec![vec![default; cols]; rows],
        }
    }
    pub fn new_void(unit_shape: UnitShape, rows: usize, cols: usize) -> Self {
        Maze::new(unit_shape, rows, cols, VOID)
    }

    pub fn from(unit_shape: UnitShape, data: MazeData) -> Self {
        Self { unit_shape, data }
    }

    pub fn rows(&self) -> usize {
        self.data.len()
    }

    pub fn cols(&self) -> usize {
        self.data.first().map_or(0, Vec::len)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::maze::{NodeFactory, OPEN, UnitShape, VOID};

    #[test]
    fn maze_new_and_new_void_have_expected_dimensions() {
        let m = Maze::new(UnitShape::Square, 2, 3, OPEN);
        assert_eq!(m.rows(), 2);
        assert_eq!(m.cols(), 3);

        let v = Maze::new_void(UnitShape::Hexagon, 3, 2);
        assert_eq!(v.rows(), 3);
        assert_eq!(v.cols(), 2);
        assert_eq!(v.data[0][0], VOID);
    }

    #[test]
    fn maze_index_and_mut_index_work() {
        let mut m = Maze::new(UnitShape::Square, 2, 2, VOID);
        let n = NodeFactory::new(2, 2).at(1, 1).unwrap();
        m[n] = OPEN;
        assert_eq!(m[n], OPEN);
    }

    #[test]
    fn maze_from_uses_provided_data() {
        let m = Maze::from(UnitShape::Square, vec![vec![1, 0], vec![0, 1]]);
        assert_eq!(m.rows(), 2);
        assert_eq!(m.cols(), 2);
    }
}
