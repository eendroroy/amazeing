use crate::helper::reconstruct_path;
use crate::maze::matrix::neighbour::neighbours_open;
use crate::maze::matrix::Maze;
use crate::solver::matrix::common::validate;
use crate::structure::queue::Queue;
use crate::structure::stack::Stack;
use crate::structure::structure_traits::DataStorage;
use crate::{Node, Tracer};
use std::collections::{BTreeMap, HashMap};

fn traverse(
    maze: &Maze,
    source: Node,
    destination: Node,
    storage: &mut dyn DataStorage<Node>,
    tracer: &mut Option<Tracer>,
) -> Vec<Node> {
    validate(maze, source, destination);

    let mut visited: HashMap<Node, bool> = HashMap::with_capacity(maze.rows() * maze.cols());
    let mut parent: BTreeMap<Node, Node> = BTreeMap::new();

    storage.push(source);

    while let Some(current) = storage.pop() {
        visited.insert(current, true);

        if let Some(trace) = tracer {
            trace.push(reconstruct_path(current, &parent));
        }

        if current == destination {
            let path = reconstruct_path(destination, &parent);
            return path;
        }

        for next in neighbours_open(maze, current, None) {
            if visited.get(&next).is_none() || visited.get(&next).unwrap().clone() == false {
                parent.insert(next, current);
                storage.push(next);
            }
        }
    }

    if let Some(trace) = tracer {
        trace.push(vec![source, destination]);
    }

    Vec::new()
}

pub fn bfs(maze: &Maze, start: Node, end: Node, tracer: &mut Option<Tracer>) -> Vec<Node> {
    let mut queue = Queue::<Node>::new();
    traverse(maze, start, end, &mut queue, tracer)
}

pub fn dfs(maze: &Maze, start: Node, end: Node, tracer: &mut Option<Tracer>) -> Vec<Node> {
    let mut queue = Stack::<Node>::new();
    traverse(maze, start, end, &mut queue, tracer)
}
