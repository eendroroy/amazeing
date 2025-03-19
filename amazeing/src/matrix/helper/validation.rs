use crate::matrix::maze::maze::Maze;
use crate::matrix::types::Node;

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
