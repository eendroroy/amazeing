use crate::helper::{validate_node, validate_node_open};
use crate::maze::matrix::Maze;
use crate::Node;

pub(crate) fn validate(maze: &Maze, source: Node, destination: Node) {
    validate_node(maze, source);
    validate_node_open(maze, source);

    validate_node(maze, destination);
    validate_node_open(maze, destination);
}
