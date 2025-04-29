use super::helper::{reconstruct_path, reconstruct_trace_path, validate};
use super::maze::Maze;
use super::{NodeHeuFn, Pop, Push, Tracer, UnitShape};
use crate::tiled::node::{DNodeWeightedForward, Node};
use std::collections::{BTreeMap, BinaryHeap, HashMap, VecDeque};

fn traverse(
    maze: &Maze,
    unit_shape: &UnitShape,
    source: Node,
    destination: Node,
    push: Push,
    pop: Pop,
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
            trace.push(reconstruct_trace_path(current, &parent));
        }

        if current == destination {
            let path = reconstruct_path(destination, &parent);
            return path;
        }

        for next in current.neighbours_open(maze, unit_shape) {
            if !visited.contains_key(&next) || !(*visited.get(&next).unwrap()) {
                parent.insert(next, current);
                push(storage, next);
            }
        }
    }

    Vec::new()
}

fn weighted_traverse(
    maze: &Maze,
    unit_shape: &UnitShape,
    source: Node,
    destination: Node,
    heu: NodeHeuFn,
    tracer: &mut Option<Tracer>,
) -> Vec<Node> {
    validate(maze, source, destination);

    let capacity = maze.rows() * maze.cols();

    let mut storage: BinaryHeap<DNodeWeightedForward> = BinaryHeap::with_capacity(capacity);
    let mut visited: HashMap<Node, bool> = HashMap::with_capacity(capacity);
    let mut parent: BTreeMap<Node, Node> = BTreeMap::new();

    storage.push(DNodeWeightedForward {
        node: source,
        cost: maze[source] as u32,
        heu_cost: maze[source] as u32 + heu(source, destination),
    });

    while let Some(node) = storage.pop() {
        let (current, cost, _) = (node.node, node.cost, node.heu_cost);
        visited.insert(current, true);

        if let Some(trace) = tracer {
            trace.push(reconstruct_trace_path(current, &parent));
        }

        if current == destination {
            let path = reconstruct_path(destination, &parent);
            return path;
        }

        for next in current.neighbours_open(maze, unit_shape) {
            if !visited.contains_key(&next) || !(*visited.get(&next).unwrap()) {
                parent.insert(next, current);
                storage.push(DNodeWeightedForward {
                    node: next,
                    cost: cost + maze[next] as u32,
                    heu_cost: cost + maze[next] as u32 + heu(next, destination),
                });
            }
        }
    }

    Vec::new()
}

pub fn bfs(
    maze: &Maze,
    unit_shape: &UnitShape,
    source: Node,
    destination: Node,
    tracer: &mut Option<Tracer>,
) -> Vec<Node> {
    let push = |s: &mut VecDeque<Node>, n: Node| s.push_back(n);
    let pop = |s: &mut VecDeque<Node>| s.pop_front();
    traverse(maze, unit_shape, source, destination, push, pop, tracer)
}

pub fn dfs(
    maze: &Maze,
    unit_shape: &UnitShape,
    source: Node,
    destination: Node,
    tracer: &mut Option<Tracer>,
) -> Vec<Node> {
    let push = |s: &mut VecDeque<Node>, n: Node| s.push_back(n);
    let pop = |s: &mut VecDeque<Node>| s.pop_back();

    traverse(maze, unit_shape, source, destination, push, pop, tracer)
}

pub fn a_star(
    maze: &Maze,
    unit_shape: &UnitShape,
    source: Node,
    destination: Node,
    heu: NodeHeuFn,
    tracer: &mut Option<Tracer>,
) -> Vec<Node> {
    weighted_traverse(maze, unit_shape, source, destination, heu, tracer)
}
