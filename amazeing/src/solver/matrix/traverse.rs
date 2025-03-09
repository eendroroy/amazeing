use crate::maze::matrix::{neighbours, Maze, D, L, R, U};
use crate::solver::matrix::common::{reconstruct_path, validate};
use crate::structure::queue::Queue;
use crate::structure::stack::Stack;
use crate::structure::structure_traits::DataStorage;
use crate::structure::DNode;
use std::collections::{BTreeMap, HashMap};

fn traverse(
    maze: &Maze,
    source: DNode,
    destination: DNode,
    storage: &mut dyn DataStorage<DNode>,
    tracer: &mut Option<Vec<Vec<DNode>>>,
) -> Vec<DNode> {
    validate(maze, source, destination);

    let mut visited: HashMap<DNode, bool> = HashMap::with_capacity(maze.rows() * maze.cols());
    let mut parent: BTreeMap<DNode, DNode> = BTreeMap::new();

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

        for next in neighbours(maze, current, &vec![R, D, L, U]) {
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

pub fn bfs(
    maze: &Maze,
    start: DNode,
    end: DNode,
    tracer: &mut Option<Vec<Vec<DNode>>>,
) -> Vec<DNode> {
    let mut queue = Queue::<DNode>::new();
    traverse(maze, start, end, &mut queue, tracer)
}

pub fn dfs(
    maze: &Maze,
    start: DNode,
    end: DNode,
    tracer: &mut Option<Vec<Vec<DNode>>>,
) -> Vec<DNode> {
    let mut queue = Stack::<DNode>::new();
    traverse(maze, start, end, &mut queue, tracer)
}
