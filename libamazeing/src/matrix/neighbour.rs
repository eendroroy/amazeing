use super::types::{NeighbourFn, Node};
use super::{Maze, NavMode};
use std::iter::Iterator;

pub(crate) const LEFT: NeighbourFn = |of| if of.1 > 0 { Some((of.0, of.1 - 1)) } else { None };
pub(crate) const RIGHT: NeighbourFn = |of| Some((of.0, of.1 + 1));
pub(crate) const UP: NeighbourFn = |of| if of.0 > 0 { Some((of.0 - 1, of.1)) } else { None };
pub(crate) const DOWN: NeighbourFn = |of| Some((of.0 + 1, of.1));
pub(crate) const LEFT_UP: NeighbourFn = |of| if of > (0, 0) { Some((of.0 - 1, of.1 - 1)) } else { None };
pub(crate) const LEFT_DOWN: NeighbourFn = |of| if of.1 > 0 { Some((of.0 + 1, of.1 - 1)) } else { None };
pub(crate) const RIGHT_UP: NeighbourFn = |of| if of.0 > 0 { Some((of.0 - 1, of.1 + 1)) } else { None };
pub(crate) const RIGHT_DOWN: NeighbourFn = |of| Some((of.0 + 1, of.1 + 1));

pub(crate) fn neighbours(maze: &Maze, pos: Node, nav: NavMode) -> Vec<Node> {
    match nav {
        NavMode::Square => vec![RIGHT, DOWN, LEFT, UP],
        NavMode::Hexagonal => {
            if pos.0 % 2 == 0 {
                vec![RIGHT, DOWN, LEFT_DOWN, LEFT, LEFT_UP, UP]
            } else {
                vec![RIGHT, RIGHT_DOWN, DOWN, LEFT, UP, RIGHT_UP]
            }
        }
    }
    .iter()
    .filter_map(|i| i(pos))
    .filter(|p| p.0 < maze.rows() && p.1 < maze.cols())
    .collect()
}

pub(crate) fn neighbours_open(maze: &Maze, pos: Node, nav: NavMode) -> Vec<Node> {
    neighbours(maze, pos, nav)
        .into_iter()
        .filter(|p| maze[*p] > 0)
        .collect()
}

pub(crate) fn neighbours_block(maze: &Maze, pos: Node, nav: NavMode) -> Vec<Node> {
    neighbours(maze, pos, nav)
        .into_iter()
        .filter(|p| maze[*p] < 1)
        .collect()
}
