use super::types::Node;
use std::ops::{Index, IndexMut};

/// A struct representing a maze.
///
/// # Attributes
/// - `data`: A 2D vector containing the maze data.
#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Maze {
    pub(crate) data: Vec<Vec<u32>>,
}

impl Maze {
    /// Creates a new `Maze` from the given data.
    ///
    /// # Arguments
    /// * `data` - A 2D vector containing the maze data.
    ///
    /// # Returns
    /// A new `Maze` instance with the provided data.
    pub fn from(data: Vec<Vec<u32>>) -> Self {
        Self { data }
    }

    /// Sets the data for the `Maze`.
    ///
    /// # Arguments
    /// * `data` - A 2D vector containing the new maze data.
    pub fn set_data(&mut self, data: Vec<Vec<u32>>) {
        self.data = data
    }

    /// Returns the number of rows in the `Maze`.
    ///
    /// # Returns
    /// The number of rows in the maze.
    pub fn rows(&self) -> usize {
        self.data.iter().len()
    }

    /// Returns the number of columns in the `Maze`.
    ///
    /// # Returns
    /// The number of columns in the maze.
    pub fn cols(&self) -> usize {
        self.data.first().unwrap().iter().len()
    }
}

/// Implements indexing for `Maze` using `Node`.
impl Index<Node> for Maze {
    type Output = u32;

    /// Returns a reference to the value at the given `Node` index.
    ///
    /// # Arguments
    /// * `index` - A `Node` representing the position in the maze.
    ///
    /// # Returns
    /// A reference to the value at the specified index.
    fn index(&self, index: Node) -> &Self::Output {
        &self.data[index.0][index.1]
    }
}

/// Implements mutable indexing for `Maze` using `Node`.
impl IndexMut<Node> for Maze {
    /// Returns a mutable reference to the value at the given `Node` index.
    ///
    /// # Arguments
    /// * `index` - A `Node` representing the position in the maze.
    ///
    /// # Returns
    /// A mutable reference to the value at the specified index.
    fn index_mut(&mut self, index: Node) -> &mut Self::Output {
        &mut self.data[index.0][index.1]
    }
}
