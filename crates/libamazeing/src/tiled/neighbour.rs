use super::types::{NeighbourFn, Node};
use super::{BLOCK, Maze, OPEN, UnitShape};
use std::iter::Iterator;

pub const LEFT: NeighbourFn = |of| if of.1 > 0 { Some((of.0, of.1 - 1)) } else { None };
pub const RIGHT: NeighbourFn = |of| Some((of.0, of.1 + 1));
pub const UP: NeighbourFn = |of| if of.0 > 0 { Some((of.0 - 1, of.1)) } else { None };
pub const DOWN: NeighbourFn = |of| Some((of.0 + 1, of.1));
pub const LEFT_UP: NeighbourFn = |of| if of.0 > 0 && of.1 > 0 { Some((of.0 - 1, of.1 - 1)) } else { None };
pub const LEFT_DOWN: NeighbourFn = |of| if of.1 > 0 { Some((of.0 + 1, of.1 - 1)) } else { None };
pub const RIGHT_UP: NeighbourFn = |of| if of.0 > 0 { Some((of.0 - 1, of.1 + 1)) } else { None };
pub const RIGHT_DOWN: NeighbourFn = |of| Some((of.0 + 1, of.1 + 1));

pub(crate) fn neighbours(maze: &Maze, pos: Node, unit_shape: &UnitShape) -> Vec<Node> {
    match unit_shape {
        UnitShape::Triangle => match pos.0 % 4 {
            0 => vec![DOWN, LEFT_DOWN, UP],
            1 => vec![RIGHT_UP, DOWN, UP],
            2 => vec![RIGHT_DOWN, DOWN, UP],
            3 => vec![UP, DOWN, LEFT_UP],
            _ => unreachable!(),
        },
        UnitShape::Square => vec![RIGHT, DOWN, LEFT, UP],
        UnitShape::Hexagon | UnitShape::Circle => {
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

pub(crate) fn neighbours_open(maze: &Maze, pos: Node, unit_shape: &UnitShape) -> Vec<Node> {
    neighbours(maze, pos, unit_shape)
        .into_iter()
        .filter(|p| maze[*p] == OPEN)
        .collect()
}

pub(crate) fn neighbours_block(maze: &Maze, pos: Node, unit_shape: &UnitShape) -> Vec<Node> {
    neighbours(maze, pos, unit_shape)
        .into_iter()
        .filter(|p| maze[*p] == BLOCK)
        .collect()
}
