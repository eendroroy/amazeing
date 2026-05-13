use super::helper::{reconstruct_path, reconstruct_trace_path, validate};
use super::structure::Maze;
use super::{NodeHeuFn, Pop, Push, Trace, Tracer};
use crate::maze::node::{DNodeWeightedForward, Node};
use rand::prelude::SliceRandom;
use rand::rng;
use std::collections::{BTreeMap, BinaryHeap, HashMap, VecDeque};

fn traverse(
    maze: &Maze,
    source: Node,
    destination: Node,
    push: Push,
    pop: Pop,
    tracer: &mut Option<Tracer>,
) -> Vec<Node> {
    let mut noop = |_| {};
    traverse_emit(maze, source, destination, push, pop, tracer, &mut noop)
}

fn traverse_emit(
    maze: &Maze,
    source: Node,
    destination: Node,
    push: Push,
    pop: Pop,
    tracer: &mut Option<Tracer>,
    emit: &mut dyn FnMut(Trace),
) -> Vec<Node> {
    let storage = &mut VecDeque::new();
    validate(maze, source, destination);

    let mut visited: HashMap<Node, bool> = HashMap::with_capacity(maze.rows() * maze.cols());
    let mut parent: BTreeMap<Node, Node> = BTreeMap::new();

    push(storage, source);

    while let Some(current) = pop(storage) {
        visited.insert(current, true);

        let step = reconstruct_trace_path(current, &parent);
        if let Some(trace) = tracer {
            trace.push(step.clone());
        }
        emit(step);

        if current == destination {
            let path = reconstruct_path(destination, &parent);
            return path;
        }

        for next in current.neighbours_open(maze, &maze.unit_shape) {
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
    source: Node,
    destination: Node,
    heu: NodeHeuFn,
    tracer: &mut Option<Tracer>,
) -> Vec<Node> {
    let mut noop = |_| {};
    weighted_traverse_emit(maze, source, destination, heu, tracer, &mut noop)
}

fn weighted_traverse_emit(
    maze: &Maze,
    source: Node,
    destination: Node,
    heu: NodeHeuFn,
    tracer: &mut Option<Tracer>,
    emit: &mut dyn FnMut(Trace),
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

        let step = reconstruct_trace_path(current, &parent);
        if let Some(trace) = tracer {
            trace.push(step.clone());
        }
        emit(step);

        if current == destination {
            let path = reconstruct_path(destination, &parent);
            return path;
        }

        for next in current.neighbours_open(maze, &maze.unit_shape) {
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

pub fn bfs(maze: &Maze, source: Node, destination: Node, tracer: &mut Option<Tracer>) -> Vec<Node> {
    let push = |s: &mut VecDeque<Node>, n: Node| s.push_back(n);
    let pop = |s: &mut VecDeque<Node>| s.pop_front();
    traverse(maze, source, destination, push, pop, tracer)
}

pub fn bfs_stream(maze: &Maze, source: Node, destination: Node, emit: &mut dyn FnMut(Trace)) -> Vec<Node> {
    let push = |s: &mut VecDeque<Node>, n: Node| s.push_back(n);
    let pop = |s: &mut VecDeque<Node>| s.pop_front();
    let mut tracer = None;
    traverse_emit(maze, source, destination, push, pop, &mut tracer, emit)
}

pub fn dfs(maze: &Maze, source: Node, destination: Node, tracer: &mut Option<Tracer>) -> Vec<Node> {
    let push = |s: &mut VecDeque<Node>, n: Node| s.push_back(n);
    let pop = |s: &mut VecDeque<Node>| s.pop_back();

    traverse(maze, source, destination, push, pop, tracer)
}

pub fn dfs_stream(maze: &Maze, source: Node, destination: Node, emit: &mut dyn FnMut(Trace)) -> Vec<Node> {
    let push = |s: &mut VecDeque<Node>, n: Node| s.push_back(n);
    let pop = |s: &mut VecDeque<Node>| s.pop_back();
    let mut tracer = None;

    traverse_emit(maze, source, destination, push, pop, &mut tracer, emit)
}

fn iddfs_depth_limited(
    maze: &Maze,
    source: Node,
    destination: Node,
    max_depth: usize,
    visited: &mut HashMap<Node, bool>,
    parent: &mut BTreeMap<Node, Node>,
    tracer: &mut Option<Tracer>,
    emit: &mut dyn FnMut(Trace),
) -> Option<Vec<Node>> {
    let mut stack: Vec<(Node, usize)> = vec![(source, 0)];
    let mut found_path: Option<Vec<Node>> = None;

    while let Some((current, depth)) = stack.pop() {
        if !visited.contains_key(&current) || !(*visited.get(&current).unwrap()) {
            visited.insert(current, true);

            let step = reconstruct_trace_path(current, parent);
            if let Some(trace) = tracer {
                trace.push(step.clone());
            }
            emit(step);

            if current == destination {
                let path = reconstruct_path(destination, parent);
                found_path = Some(path);
                break;
            }

            if depth < max_depth {
                for next in current.neighbours_open(maze, &maze.unit_shape) {
                    if !visited.contains_key(&next) || !(*visited.get(&next).unwrap()) {
                        parent.insert(next, current);
                        stack.push((next, depth + 1));
                    }
                }
            }
        }
    }

    found_path
}

pub fn iddfs(maze: &Maze, source: Node, destination: Node, tracer: &mut Option<Tracer>) -> Vec<Node> {
    let mut noop = |_| {};
    iddfs_emit(maze, source, destination, tracer, &mut noop)
}

pub fn iddfs_emit(
    maze: &Maze,
    source: Node,
    destination: Node,
    tracer: &mut Option<Tracer>,
    emit: &mut dyn FnMut(Trace),
) -> Vec<Node> {
    validate(maze, source, destination);

    let mut visited: HashMap<Node, bool> = HashMap::with_capacity(maze.rows() * maze.cols());
    let mut parent: BTreeMap<Node, Node> = BTreeMap::new();

    // Try increasing depth limits
    for depth_limit in 0..=(maze.rows() * maze.cols()) {
        visited.clear();
        parent.clear();

        if let Some(path) = iddfs_depth_limited(maze, source, destination, depth_limit, &mut visited, &mut parent, tracer, emit) {
            if !path.is_empty() {
                return path;
            }
        }
    }

    Vec::new()
}

pub fn iddfs_stream(maze: &Maze, source: Node, destination: Node, emit: &mut dyn FnMut(Trace)) -> Vec<Node> {
    let mut tracer = None;
    iddfs_emit(maze, source, destination, &mut tracer, emit)
}

pub fn aldous_broder(maze: &Maze, source: Node, destination: Node, tracer: &mut Option<Tracer>) -> Vec<Node> {
    let mut noop = |_| {};
    aldous_broder_emit(maze, source, destination, tracer, &mut noop)
}

pub fn aldous_broder_stream(
    maze: &Maze,
    source: Node,
    destination: Node,
    emit: &mut dyn FnMut(Trace),
) -> Vec<Node> {
    let mut tracer = None;
    aldous_broder_emit(maze, source, destination, &mut tracer, emit)
}

fn aldous_broder_emit(
    maze: &Maze,
    source: Node,
    destination: Node,
    tracer: &mut Option<Tracer>,
    emit: &mut dyn FnMut(Trace),
) -> Vec<Node> {
    validate(maze, source, destination);

    let mut visited: HashMap<Node, bool> = HashMap::with_capacity(maze.rows() * maze.cols());
    let mut parent: BTreeMap<Node, Node> = BTreeMap::new();
    let mut current = source;

    visited.insert(source, true);

    let first_step = reconstruct_trace_path(source, &parent);
    if let Some(trace) = tracer {
        trace.push(first_step.clone());
    }
    emit(first_step);

    if source == destination {
        return vec![source];
    }

    let max_steps = maze
        .rows()
        .saturating_mul(maze.cols())
        .saturating_mul(maze.rows())
        .saturating_mul(maze.cols())
        .saturating_mul(64);

    for _ in 0..max_steps {
        let mut options = current.neighbours_open(maze, &maze.unit_shape);
        if options.is_empty() {
            return Vec::new();
        }
        options.shuffle(&mut rng());
        let next = options[0];

        if !visited.contains_key(&next) || !(*visited.get(&next).unwrap()) {
            visited.insert(next, true);
            parent.insert(next, current);
        }

        current = next;
        let step = reconstruct_trace_path(current, &parent);
        if let Some(trace) = tracer {
            trace.push(step.clone());
        }
        emit(step);

        if current == destination {
            return reconstruct_path(destination, &parent);
        }
    }

    Vec::new()
}

pub fn a_star(maze: &Maze, source: Node, destination: Node, heu: NodeHeuFn, tracer: &mut Option<Tracer>) -> Vec<Node> {
    weighted_traverse(maze, source, destination, heu, tracer)
}

pub fn a_star_stream(
    maze: &Maze,
    source: Node,
    destination: Node,
    heu: NodeHeuFn,
    emit: &mut dyn FnMut(Trace),
) -> Vec<Node> {
    let mut tracer = None;
    weighted_traverse_emit(maze, source, destination, heu, &mut tracer, emit)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::maze::heuristics::manhattan_heuristic;
    use crate::maze::{NodeFactory, OPEN, UnitShape};

    fn open_grid(rows: usize, cols: usize) -> Maze {
        Maze::new(UnitShape::Square, rows, cols, OPEN)
    }

    #[test]
    fn solver_finds_path_for_all_algorithms() {
        let maze = open_grid(4, 4);
        let f = NodeFactory::new(4, 4);
        let source = f.at(0, 0).unwrap();
        let destination = f.at(3, 3).unwrap();

        let bfs_path = bfs(&maze, source, destination, &mut None);
        assert_eq!(bfs_path.first(), Some(&source));
        assert_eq!(bfs_path.last(), Some(&destination));

        let dfs_path = dfs(&maze, source, destination, &mut None);
        assert_eq!(dfs_path.first(), Some(&source));
        assert_eq!(dfs_path.last(), Some(&destination));

        let iddfs_path = iddfs(&maze, source, destination, &mut None);
        assert_eq!(iddfs_path.first(), Some(&source));
        assert_eq!(iddfs_path.last(), Some(&destination));

        let astar_path = a_star(&maze, source, destination, manhattan_heuristic, &mut None);
        assert_eq!(astar_path.first(), Some(&source));
        assert_eq!(astar_path.last(), Some(&destination));

        let aldous_broder_path = aldous_broder(&maze, source, destination, &mut None);
        assert_eq!(aldous_broder_path.first(), Some(&source));
        assert_eq!(aldous_broder_path.last(), Some(&destination));
    }

    #[test]
    fn stream_variants_emit_trace_steps() {
        let maze = open_grid(3, 3);
        let f = NodeFactory::new(3, 3);
        let source = f.at(0, 0).unwrap();
        let destination = f.at(2, 2).unwrap();

        let mut bfs_steps = 0usize;
        let mut emit_bfs = |_| bfs_steps += 1;
        let path = bfs_stream(&maze, source, destination, &mut emit_bfs);
        assert!(bfs_steps > 0);
        assert!(!path.is_empty());

        let mut dfs_steps = 0usize;
        let mut emit_dfs = |_| dfs_steps += 1;
        let path = dfs_stream(&maze, source, destination, &mut emit_dfs);
        assert!(dfs_steps > 0);
        assert!(!path.is_empty());

        let mut iddfs_steps = 0usize;
        let mut emit_iddfs = |_| iddfs_steps += 1;
        let path = iddfs_stream(&maze, source, destination, &mut emit_iddfs);
        assert!(iddfs_steps > 0);
        assert!(!path.is_empty());

        let mut astar_steps = 0usize;
        let mut emit_astar = |_| astar_steps += 1;
        let path = a_star_stream(&maze, source, destination, manhattan_heuristic, &mut emit_astar);
        assert!(astar_steps > 0);
        assert!(!path.is_empty());

        let mut aldous_broder_steps = 0usize;
        let mut emit_aldous_broder = |_| aldous_broder_steps += 1;
        let path = aldous_broder_stream(&maze, source, destination, &mut emit_aldous_broder);
        assert!(aldous_broder_steps > 0);
        assert!(!path.is_empty());
    }
}
