use crate::matrix::helper::reconstruct_path::reconstruct_path;
use crate::matrix::maze::maze::Maze;
use crate::matrix::maze::neighbour::neighbours_open;
use crate::matrix::solver::common::validate;
use crate::matrix::types::{Node, Tracer};
use std::collections::{BTreeMap, HashMap, VecDeque};

fn traverse(
    maze: &Maze,
    source: Node,
    destination: Node,
    push: fn(&mut VecDeque<Node>, Node),
    pop: fn(&mut VecDeque<Node>) -> Option<Node>,
    tracer: &mut Option<Tracer>,
) -> Vec<Node> {
    let storage = &mut VecDeque::new();
    validate(maze, source, destination);

    let mut visited: HashMap<Node, bool> = HashMap::with_capacity(maze.rows() * maze.cols());
    let mut parent: BTreeMap<Node, Node> = BTreeMap::new();

    push(storage, source);

    while let Some(current) = pop(storage) {
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
                push(storage, next);
            }
        }
    }

    if let Some(trace) = tracer {
        trace.push(vec![source, destination]);
    }

    Vec::new()
}

pub fn bfs(maze: &Maze, start: Node, end: Node, tracer: &mut Option<Tracer>) -> Vec<Node> {
    let push = |s: &mut VecDeque<Node>, n: Node| s.push_back(n);
    let pop = |s: &mut VecDeque<Node>| s.pop_front();
    traverse(maze, start, end, push, pop, tracer)
}

pub fn dfs(maze: &Maze, start: Node, end: Node, tracer: &mut Option<Tracer>) -> Vec<Node> {
    let push = |s: &mut VecDeque<Node>, n: Node| s.push_back(n);
    let pop = |s: &mut VecDeque<Node>| s.pop_back();

    traverse(maze, start, end, push, pop, tracer)
}
