use crate::square::maze::Maze;
use std::iter::Iterator;

type Neighbour = fn((usize, usize)) -> (usize, usize);

const D: Neighbour = |of| -> (usize, usize) { (of.0 + 1, of.1) };
const R: Neighbour = |of| -> (usize, usize) { (of.0, of.1 + 1) };
const L: Neighbour = |of| -> (usize, usize) { (of.0, of.1 - 1) };
const U: Neighbour = |of| -> (usize, usize) { (of.0 - 1, of.1) };

pub(crate) fn neighbours<const ROWS: usize, const COLS: usize>(
    maze: &Maze<ROWS, COLS>,
    pos: (usize, usize),
) -> Vec<(usize, usize)> {
    match pos {
        (r, c) if (r, c) == (0, 0) => vec![D, R],        // top-left
        (r, c) if (r, c) == (ROWS - 1, 0) => vec![R, U], // bottom-left
        (r, c) if (r, c) == (ROWS - 1, COLS - 1) => vec![L, U], // bottom-right
        (r, c) if (r, c) == (0, COLS - 1) => vec![D, L], // top-right
        (r, _) if r == 0 => vec![D, R, L],               // top
        (_, c) if c == 0 => vec![D, R, U],               // left
        (r, _) if r == ROWS - 1 => vec![R, L, U],        // bottom
        (_, c) if c == COLS - 1 => vec![D, L, U],        // right
        (_, _) => vec![D, R, L, U],                      // inner
    }
    .iter()
    .map(|i| i(pos))
    .filter(|p| maze[*p] > 0)
    .map(|p| p.clone())
    .collect()
}
