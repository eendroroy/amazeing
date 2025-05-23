use super::{Maze, Node, OPEN, Rank, Trace};
use std::collections::{BTreeMap, HashMap};

pub(crate) fn validate_node(maze: &Maze, node: Node) {
    if node.row >= maze.rows() || node.col >= maze.cols() {
        panic!(
            "Invalid node({},{}), available nodes (0,0 - {},{})",
            node.row,
            node.col,
            maze.rows() - 1,
            maze.cols() - 1
        );
    }
}

pub(crate) fn validate_node_open(maze: &Maze, node: Node) {
    if maze[node] != OPEN {
        panic!("Invalid node({},{} [{}])", node.row, node.col, maze[node])
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

pub(crate) fn reconstruct_trace_path(destination: Node, parent: &BTreeMap<Node, Node>) -> Trace {
    let mut rank = Rank::MAX;
    let mut path = HashMap::<Node, i32>::new();
    let mut current_node = destination;
    while parent.contains_key(&current_node) {
        path.insert(current_node, rank);
        rank -= 1;
        current_node = parent[&current_node];
    }
    path.insert(current_node, rank);
    path
}

pub(crate) fn validate(maze: &Maze, source: Node, destination: Node) {
    validate_node(maze, source);
    validate_node_open(maze, source);

    validate_node(maze, destination);
    validate_node_open(maze, destination);
}
