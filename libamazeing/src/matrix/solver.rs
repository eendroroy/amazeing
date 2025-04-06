use crate::matrix::helper::{reconstruct_path, reconstruct_trace_path, validate};
use crate::matrix::heuristics::dijkstra_heuristic;
use crate::matrix::maze::Maze;
use crate::matrix::neighbour::neighbours_open;
use crate::matrix::{Node, NodeHeuFn, Tracer};
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
            trace.push(reconstruct_trace_path(current, &parent));
        }

        if current == destination {
            let path = reconstruct_path(destination, &parent);
            return path;
        }

        for next in neighbours_open(maze, current, None) {
            if visited.get(&next).is_none() || !(*visited.get(&next).unwrap()) {
                parent.insert(next, current);
                push(storage, next);
            }
        }
    }

    Vec::new()
}

fn weighted_traverse(
    maze: &Maze,
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
        cost: maze[source],
        heu_cost: maze[source] + heu(source, destination),
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

        for next in neighbours_open(maze, current, None) {
            if visited.get(&next).is_none() || !(*visited.get(&next).unwrap()) {
                parent.insert(next, current);
                storage.push(DNodeWeighted {
                    node: next,
                    cost: cost + maze[next],
                    heu_cost: cost + maze[next] + heu(next, destination),
                });
            }
        }
    }

    Vec::new()
}

/// Performs a breadth-first search (BFS) on the maze from the start node to the end node.
///
/// # Arguments
///
/// * `maze` - A reference to the maze to be traversed.
/// * `start` - The starting node.
/// * `end` - The destination node.
/// * `tracer` - An optional tracer to record the path.
///
/// # Returns
///
/// A vector of nodes representing the path from start to end.
pub fn bfs(maze: &Maze, start: Node, end: Node, tracer: &mut Option<Tracer>) -> Vec<Node> {
    let push = |s: &mut VecDeque<Node>, n: Node| s.push_back(n);
    let pop = |s: &mut VecDeque<Node>| s.pop_front();
    traverse(maze, start, end, push, pop, tracer)
}

/// Performs a depth-first search (DFS) on the maze from the start node to the end node.
///
/// # Arguments
///
/// * `maze` - A reference to the maze to be traversed.
/// * `start` - The starting node.
/// * `end` - The destination node.
/// * `tracer` - An optional tracer to record the path.
///
/// # Returns
///
/// A vector of nodes representing the path from start to end.
pub fn dfs(maze: &Maze, start: Node, end: Node, tracer: &mut Option<Tracer>) -> Vec<Node> {
    let push = |s: &mut VecDeque<Node>, n: Node| s.push_back(n);
    let pop = |s: &mut VecDeque<Node>| s.pop_back();

    traverse(maze, start, end, push, pop, tracer)
}

/// Performs Dijkstra's algorithm on the maze from the start node to the end node.
///
/// # Arguments
///
/// * `maze` - A reference to the maze to be traversed.
/// * `start` - The starting node.
/// * `end` - The destination node.
/// * `tracer` - An optional tracer to record the path.
///
/// # Returns
///
/// A vector of nodes representing the path from start to end.
pub fn dijkstra(maze: &Maze, start: Node, end: Node, tracer: &mut Option<Tracer>) -> Vec<Node> {
    weighted_traverse(maze, start, end, dijkstra_heuristic, tracer)
}

/// Performs the A* algorithm on the maze from the start node to the end node.
///
/// # Arguments
///
/// * `maze` - A reference to the maze to be traversed.
/// * `start` - The starting node.
/// * `end` - The destination node.
/// * `heu` - A heuristic function to estimate the cost from a node to the destination.
/// * `tracer` - An optional tracer to record the path.
///
/// # Returns
///
/// A vector of nodes representing the path from start to end.
pub fn a_star(maze: &Maze, start: Node, end: Node, heu: NodeHeuFn, tracer: &mut Option<Tracer>) -> Vec<Node> {
    weighted_traverse(maze, start, end, heu, tracer)
}
