use crate::matrix::Maze;
use crate::matrix::types::{NeighbourFn, Node};
use std::iter::Iterator;

pub(crate) const DOWN: NeighbourFn = |of| -> Option<Node> { Some((of.0 + 1, of.1)) };
pub(crate) const RIGHT: NeighbourFn = |of| -> Option<Node> { Some((of.0, of.1 + 1)) };
pub(crate) const LEFT: NeighbourFn = |of| -> Option<Node> { if of.1 > 0 { Some((of.0, of.1 - 1)) } else { None } };
pub(crate) const UP: NeighbourFn = |of| -> Option<Node> { if of.0 > 0 { Some((of.0 - 1, of.1)) } else { None } };

type Traversal = Option<Vec<NeighbourFn>>;

pub(crate) fn neighbours(maze: &Maze, pos: Node, directions: Traversal) -> Vec<Node> {
    let direction_list = directions.unwrap_or_else(|| vec![RIGHT, DOWN, LEFT, UP]);

    direction_list
        .iter()
        .map(|i| i(pos))
        .filter(|p| p.is_some())
        .map(|p| p.unwrap())
        .filter(|p| p.0 < maze.rows() && p.1 < maze.cols())
        .map(|p| p.clone())
        .collect()
}

pub(crate) fn neighbours_open(maze: &Maze, pos: Node, directions: Traversal) -> Vec<Node> {
    neighbours(maze, pos, directions)
        .into_iter()
        .filter(|p| maze[*p] > 0)
        .collect()
}

pub(crate) fn neighbours_block(maze: &Maze, pos: Node, directions: Traversal) -> Vec<Node> {
    neighbours(maze, pos, directions)
        .into_iter()
        .filter(|p| maze[*p] < 1)
        .collect()
}
