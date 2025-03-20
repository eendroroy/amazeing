use crate::matrix::types::Node;
use std::collections::BTreeMap;
use crate::matrix::Maze;

pub(crate) fn validate_node(maze: &Maze, node: Node) {
    if node.0 >= maze.rows() || node.1 >= maze.cols() {
        panic!(
            "Invalid node({},{}), available nodes (0,0 - {},{})",
            node.0,
            node.1,
            maze.rows() - 1,
            maze.cols() - 1
        );
    }
}

pub(crate) fn validate_node_open(maze: &Maze, node: Node) {
    if maze[node] < 1 {
        panic!("Invalid node({},{} [{}])", node.0, node.1, maze[node])
    }
}

pub(crate) fn reconstruct_path(destination: Node, parent: &BTreeMap<Node, Node>) -> Vec<Node> {
    let mut path = Vec::<Node>::new();
    let mut current_node = destination;
    while parent.contains_key(&current_node) {
        path.push(current_node);
        current_node = parent[&current_node];
    }
    path.push(current_node);
    path.reverse();
    path
}

pub(crate) fn validate(maze: &Maze, source: Node, destination: Node) {
    validate_node(maze, source);
    validate_node_open(maze, source);

    validate_node(maze, destination);
    validate_node_open(maze, destination);
}
