use crate::matrix::helper::validation::{validate_node, validate_node_open};
use crate::matrix::maze::maze::Maze;
use crate::matrix::types::Node;

pub(crate) fn validate(maze: &Maze, source: Node, destination: Node) {
    validate_node(maze, source);
    validate_node_open(maze, source);

    validate_node(maze, destination);
    validate_node_open(maze, destination);
}
