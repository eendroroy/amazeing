use crate::square::maze::Maze;
use std::iter::Iterator;

pub(crate) type Node = (usize, usize);
pub(crate) type Neighbour = fn((usize, usize)) -> Option<(usize, usize)>;

pub(crate) const D: Neighbour = |of| -> Option<Node> { Some((of.0 + 1, of.1)) };
pub(crate) const R: Neighbour = |of| -> Option<Node> { Some((of.0, of.1 + 1)) };
pub(crate) const L: Neighbour = |of| -> Option<Node> {
    if of.1 > 0 {
        Some((of.0, of.1 - 1))
    } else {
        None
    }
};
pub(crate) const U: Neighbour = |of| -> Option<Node> {
    if of.0 > 0 {
        Some((of.0 - 1, of.1))
    } else {
        None
    }
};

pub(crate) fn neighbours<const ROWS: usize, const COLS: usize>(
    maze: &Maze<ROWS, COLS>,
    pos: (usize, usize),
    direction: &Vec<Neighbour>,
) -> Vec<(usize, usize)> {
    direction
        .iter()
        .map(|i| i(pos))
        .filter(|p| p.is_some())
        .map(|p| p.unwrap())
        .filter(|p| p.0 < ROWS && p.1 < COLS)
        .filter(|p| maze[*p] > 0)
        .map(|p| p.clone())
        .collect()
}
