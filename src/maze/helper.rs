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
    let mut nodes = Vec::with_capacity(parent.len() + 1);
    let mut current = destination;
    nodes.push(current);
    while let Some(&next) = parent.get(&current) {
        current = next;
        nodes.push(current);
    }

    let mut path = HashMap::with_capacity(nodes.len());
    let mut rank = Rank::MAX;
    for &node in &nodes {
        path.insert(node, rank);
        rank -= 1;
    }
    path
}

pub(crate) fn validate(maze: &Maze, source: Node, destination: Node) {
    validate_node(maze, source);
    validate_node_open(maze, source);

    validate_node(maze, destination);
    validate_node_open(maze, destination);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::maze::{Maze, NodeFactory, OPEN, UnitShape, VOID};

    #[test]
    fn reconstruct_helpers_build_expected_sequences() {
        let f = NodeFactory::new(3, 3);
        let a = f.at(0, 0).unwrap();
        let b = f.at(0, 1).unwrap();
        let c = f.at(0, 2).unwrap();

        let mut parent = BTreeMap::new();
        parent.insert(b, a);
        parent.insert(c, b);

        let path = reconstruct_path(c, &parent);
        assert_eq!(path, vec![a, b, c]);

        let trace = reconstruct_trace_path(c, &parent);
        assert_eq!(trace.get(&c), Some(&Rank::MAX));
        assert_eq!(trace.get(&b), Some(&(Rank::MAX - 1)));
        assert_eq!(trace.get(&a), Some(&(Rank::MAX - 2)));
    }

    #[test]
    fn validate_accepts_open_source_and_destination() {
        let mut maze = Maze::new(UnitShape::Square, 3, 3, VOID);
        let f = NodeFactory::new(3, 3);
        let s = f.at(0, 0).unwrap();
        let d = f.at(0, 1).unwrap();
        maze[s] = OPEN;
        maze[d] = OPEN;
        validate(&maze, s, d);
    }

    #[test]
    #[should_panic]
    fn validate_panics_for_out_of_bounds_node() {
        let maze = Maze::new(UnitShape::Square, 2, 2, OPEN);
        let bad = NodeFactory::new(3, 3).at(2, 2).unwrap();
        validate_node(&maze, bad);
    }

    #[test]
    #[should_panic]
    fn validate_node_open_panics_for_block() {
        let maze = Maze::new(UnitShape::Square, 2, 2, VOID);
        let node = NodeFactory::new(2, 2).at(1, 1).unwrap();
        validate_node_open(&maze, node);
    }
}
