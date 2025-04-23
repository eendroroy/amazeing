use super::helper::{reconstruct_path, reconstruct_trace_path, validate};
use super::heuristics::dijkstra_heuristic;
use super::maze::Maze;
use super::neighbour::neighbours_open;
use super::{Node, NodeHeuFn, Pop, Push, Tracer, UnitShape};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap, HashMap, VecDeque};

#[derive(Debug, Clone, Eq, PartialEq)]
struct DNodeWeighted {
    pub(crate) node: Node,
    pub(crate) cost: u32,
    pub(crate) heu_cost: u32,
}

impl PartialOrd<Self> for DNodeWeighted {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DNodeWeighted {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heu_cost.cmp(&self.heu_cost)
    }
}

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

        for next in neighbours_open(maze, current, unit_shape) {
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

    let mut storage: BinaryHeap<DNodeWeighted> = BinaryHeap::with_capacity(capacity);
    let mut visited: HashMap<Node, bool> = HashMap::with_capacity(capacity);
    let mut parent: BTreeMap<Node, Node> = BTreeMap::new();

    storage.push(DNodeWeighted {
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

        for next in neighbours_open(maze, current, unit_shape) {
            if !visited.contains_key(&next) || !(*visited.get(&next).unwrap()) {
                parent.insert(next, current);
                storage.push(DNodeWeighted {
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

pub fn dijkstra(
    maze: &Maze,
    unit_shape: &UnitShape,
    source: Node,
    destination: Node,
    tracer: &mut Option<Tracer>,
) -> Vec<Node> {
    weighted_traverse(maze, unit_shape, source, destination, dijkstra_heuristic, tracer)
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
