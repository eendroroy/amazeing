use crate::solver::matrix::Maze;
use std::iter::Iterator;

pub(crate) type DNode = (usize, usize);
pub(crate) type FnNeighbour = fn(DNode) -> Option<DNode>;

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

pub(crate) fn neighbours<const ROWS: usize, const COLS: usize>(
    maze: &Maze<ROWS, COLS>,
    pos: DNode,
    direction: &Vec<FnNeighbour>,
) -> Vec<DNode> {
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
