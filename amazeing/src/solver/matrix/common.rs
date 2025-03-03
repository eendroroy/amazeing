use crate::solver::matrix::neighbour::DNode;
use crate::solver::matrix::Maze;
use std::collections::BTreeMap;

pub(crate) fn validate(maze: &Maze, source: DNode, destination: DNode) {
    if source.0 >= maze.rows()
        || source.1 >= maze.cols()
        || destination.0 >= maze.rows()
        || destination.1 >= maze.cols()
    {
        panic!(
            "Invalid start({},{}) or end({},{}) node, available nodes (0,0 - {},{})",
            source.0,
            source.1,
            destination.0,
            destination.1,
            maze.rows() - 1,
            maze.cols() - 1
        );
    }
    if maze[source] < 1 || maze[destination] < 1 {
        panic!(
            "Invalid start({},{} [{}]) or end({},{}, [{}]) node",
            source.0, source.1, maze[source], destination.0, destination.1, maze[destination]
        )
    }
}

pub(crate) fn reconstruct_path(destination: DNode, parent: &BTreeMap<DNode, DNode>) -> Vec<DNode> {
    let mut path = Vec::<DNode>::new();
    let mut current_node = destination;
    while parent.contains_key(&current_node) {
        path.push(current_node);
        current_node = parent[&current_node];
    }
    path.push(current_node);
    path.reverse();
    path
}
