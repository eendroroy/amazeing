use crate::maze::matrix::Maze;
use crate::structure::{DNode, FnNeighbour};
use std::iter::Iterator;

pub(crate) const D: FnNeighbour = |of| -> Option<DNode> { Some((of.0 + 1, of.1)) };
pub(crate) const R: FnNeighbour = |of| -> Option<DNode> { Some((of.0, of.1 + 1)) };
pub(crate) const L: FnNeighbour = |of| -> Option<DNode> {
    if of.1 > 0 {
        Some((of.0, of.1 - 1))
    } else {
        None
    }
};
pub(crate) const U: FnNeighbour = |of| -> Option<DNode> {
    if of.0 > 0 {
        Some((of.0 - 1, of.1))
    } else {
        None
    }
};

pub(crate) fn neighbours(maze: &Maze, pos: DNode) -> Vec<DNode> {
    vec![R, D, L, U]
        .iter()
        .map(|i| i(pos))
        .filter(|p| p.is_some())
        .map(|p| p.unwrap())
        .filter(|p| p.0 < maze.rows() && p.1 < maze.cols())
        .map(|p| p.clone())
        .collect()
}

pub(crate) fn neighbours_open(maze: &Maze, pos: DNode) -> Vec<DNode> {
    neighbours(maze, pos)
        .into_iter()
        .filter(|p| maze[*p] > 0)
        .collect()
}

pub(crate) fn neighbours_block(maze: &Maze, pos: DNode) -> Vec<DNode> {
    neighbours(maze, pos)
        .into_iter()
        .filter(|p| maze[*p] < 1)
        .collect()
}
