use super::helper::{reconstruct_path, reconstruct_trace_path, validate};
use super::structure::Maze;
use super::{NodeHeuFn, Pop, Push, Trace, Tracer};
use crate::maze::node::{DNodeWeightedForward, Node};
use rand::prelude::SliceRandom;
use rand::rng;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap, HashMap, VecDeque};

#[derive(Debug, Clone, Eq, PartialEq)]
struct BiAStarNode {
    node: Node,
    cost: u32,
    heu_cost: u32,
}

impl PartialOrd<Self> for BiAStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BiAStarNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heu_cost.cmp(&self.heu_cost)
    }
}

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

pub fn greedy_best_first(
    maze: &Maze,
    source: Node,
    destination: Node,
    heu: NodeHeuFn,
    tracer: &mut Option<Tracer>,
) -> Vec<Node> {
    let mut noop = |_| {};
    greedy_best_first_emit(maze, source, destination, heu, tracer, &mut noop)
}

pub fn greedy_best_first_stream(
    maze: &Maze,
    source: Node,
    destination: Node,
    heu: NodeHeuFn,
    emit: &mut dyn FnMut(Trace),
) -> Vec<Node> {
    let mut tracer = None;
    greedy_best_first_emit(maze, source, destination, heu, &mut tracer, emit)
}

fn greedy_best_first_emit(
    maze: &Maze,
    source: Node,
    destination: Node,
    heu: NodeHeuFn,
    tracer: &mut Option<Tracer>,
    emit: &mut dyn FnMut(Trace),
) -> Vec<Node> {
    validate(maze, source, destination);

    let mut storage: BinaryHeap<DNodeWeightedForward> = BinaryHeap::with_capacity(maze.rows() * maze.cols());
    let mut visited: HashMap<Node, bool> = HashMap::with_capacity(maze.rows() * maze.cols());
    let mut discovered: HashMap<Node, bool> = HashMap::with_capacity(maze.rows() * maze.cols());
    let mut parent: BTreeMap<Node, Node> = BTreeMap::new();

    discovered.insert(source, true);
    storage.push(DNodeWeightedForward {
        node: source,
        cost: 0,
        heu_cost: heu(source, destination),
    });

    while let Some(node) = storage.pop() {
        let current = node.node;
        if visited.get(&current).is_some_and(|v| *v) {
            continue;
        }

        visited.insert(current, true);
        let step = reconstruct_trace_path(current, &parent);
        if let Some(trace) = tracer {
            trace.push(step.clone());
        }
        emit(step);

        if current == destination {
            return reconstruct_path(destination, &parent);
        }

        for next in current.neighbours_open(maze, &maze.unit_shape) {
            if discovered.get(&next).is_none_or(|seen| !*seen) {
                discovered.insert(next, true);
                parent.insert(next, current);
                storage.push(DNodeWeightedForward {
                    node: next,
                    cost: 0,
                    heu_cost: heu(next, destination),
                });
            }
        }
    }

    Vec::new()
}

pub fn bidirectional_bfs(maze: &Maze, source: Node, destination: Node, tracer: &mut Option<Tracer>) -> Vec<Node> {
    let mut noop = |_| {};
    bidirectional_bfs_emit(maze, source, destination, tracer, &mut noop)
}

pub fn bidirectional_bfs_stream(
    maze: &Maze,
    source: Node,
    destination: Node,
    emit: &mut dyn FnMut(Trace),
) -> Vec<Node> {
    let mut tracer = None;
    bidirectional_bfs_emit(maze, source, destination, &mut tracer, emit)
}

fn bidirectional_bfs_emit(
    maze: &Maze,
    source: Node,
    destination: Node,
    tracer: &mut Option<Tracer>,
    emit: &mut dyn FnMut(Trace),
) -> Vec<Node> {
    validate(maze, source, destination);

    if source == destination {
        let step = reconstruct_trace_path(source, &BTreeMap::new());
        if let Some(trace) = tracer {
            trace.push(step.clone());
        }
        emit(step);
        return vec![source];
    }

    let mut queue_f: VecDeque<Node> = VecDeque::new();
    let mut queue_b: VecDeque<Node> = VecDeque::new();

    let mut visited_f: HashMap<Node, bool> = HashMap::with_capacity(maze.rows() * maze.cols());
    let mut visited_b: HashMap<Node, bool> = HashMap::with_capacity(maze.rows() * maze.cols());
    let mut parent_f: BTreeMap<Node, Node> = BTreeMap::new();
    let mut parent_b: BTreeMap<Node, Node> = BTreeMap::new();

    queue_f.push_back(source);
    queue_b.push_back(destination);
    visited_f.insert(source, true);
    visited_b.insert(destination, true);

    let source_step = reconstruct_trace_path(source, &parent_f);
    if let Some(trace) = tracer {
        trace.push(source_step.clone());
    }
    emit(source_step);

    let destination_step = reconstruct_trace_path(destination, &parent_b);
    if let Some(trace) = tracer {
        trace.push(destination_step.clone());
    }
    emit(destination_step);

    while !queue_f.is_empty() && !queue_b.is_empty() {
        let len_f = queue_f.len();
        for _ in 0..len_f {
            if let Some(current) = queue_f.pop_front() {
                for next in current.neighbours_open(maze, &maze.unit_shape) {
                    if visited_f.get(&next).is_none_or(|seen| !*seen) {
                        visited_f.insert(next, true);
                        parent_f.insert(next, current);
                        queue_f.push_back(next);

                        let step = reconstruct_trace_path(next, &parent_f);
                        if let Some(trace) = tracer {
                            trace.push(step.clone());
                        }
                        emit(step);

                        if visited_b.get(&next).is_some_and(|seen| *seen) {
                            let mut path = reconstruct_path(next, &parent_f);
                            let mut cursor = next;
                            while let Some(back_parent) = parent_b.get(&cursor) {
                                path.push(*back_parent);
                                cursor = *back_parent;
                            }
                            return path;
                        }
                    }
                }
            }
        }

        let len_b = queue_b.len();
        for _ in 0..len_b {
            if let Some(current) = queue_b.pop_front() {
                for next in current.neighbours_open(maze, &maze.unit_shape) {
                    if visited_b.get(&next).is_none_or(|seen| !*seen) {
                        visited_b.insert(next, true);
                        parent_b.insert(next, current);
                        queue_b.push_back(next);

                        let step = reconstruct_trace_path(next, &parent_b);
                        if let Some(trace) = tracer {
                            trace.push(step.clone());
                        }
                        emit(step);

                        if visited_f.get(&next).is_some_and(|seen| *seen) {
                            let mut path = reconstruct_path(next, &parent_f);
                            let mut cursor = next;
                            while let Some(back_parent) = parent_b.get(&cursor) {
                                path.push(*back_parent);
                                cursor = *back_parent;
                            }
                            return path;
                        }
                    }
                }
            }
        }
    }

    Vec::new()
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

pub fn bidirectional_a_start(
    maze: &Maze,
    source: Node,
    destination: Node,
    heu: NodeHeuFn,
    tracer: &mut Option<Tracer>,
) -> Vec<Node> {
    let mut noop = |_| {};
    bidirectional_a_start_emit(maze, source, destination, heu, tracer, &mut noop)
}

pub fn bidirectional_a_start_stream(
    maze: &Maze,
    source: Node,
    destination: Node,
    heu: NodeHeuFn,
    emit: &mut dyn FnMut(Trace),
) -> Vec<Node> {
    let mut tracer = None;
    bidirectional_a_start_emit(maze, source, destination, heu, &mut tracer, emit)
}

fn bidirectional_a_start_emit(
    maze: &Maze,
    source: Node,
    destination: Node,
    heu: NodeHeuFn,
    tracer: &mut Option<Tracer>,
    emit: &mut dyn FnMut(Trace),
) -> Vec<Node> {
    validate(maze, source, destination);

    if source == destination {
        let step = reconstruct_trace_path(source, &BTreeMap::new());
        if let Some(trace) = tracer {
            trace.push(step.clone());
        }
        emit(step);
        return vec![source];
    }

    let mut open_f: BinaryHeap<BiAStarNode> = BinaryHeap::new();
    let mut open_b: BinaryHeap<BiAStarNode> = BinaryHeap::new();

    let mut g_f: HashMap<Node, u32> = HashMap::with_capacity(maze.rows() * maze.cols());
    let mut g_b: HashMap<Node, u32> = HashMap::with_capacity(maze.rows() * maze.cols());
    let mut closed_f: HashMap<Node, bool> = HashMap::with_capacity(maze.rows() * maze.cols());
    let mut closed_b: HashMap<Node, bool> = HashMap::with_capacity(maze.rows() * maze.cols());

    let mut parent_f: BTreeMap<Node, Node> = BTreeMap::new();
    let mut parent_b: BTreeMap<Node, Node> = BTreeMap::new();

    g_f.insert(source, maze[source] as u32);
    g_b.insert(destination, maze[destination] as u32);

    open_f.push(BiAStarNode {
        node: source,
        cost: maze[source] as u32,
        heu_cost: maze[source] as u32 + heu(source, destination),
    });
    open_b.push(BiAStarNode {
        node: destination,
        cost: maze[destination] as u32,
        heu_cost: maze[destination] as u32 + heu(destination, source),
    });

    let mut best_meet: Option<Node> = None;
    let mut best_total_cost = u32::MAX;

    while !open_f.is_empty() && !open_b.is_empty() {
        if let Some(node) = open_f.pop() {
            let current = node.node;
            let cost = node.cost;
            if g_f.get(&current).is_some_and(|best| cost > *best) {
                continue;
            }

            closed_f.insert(current, true);
            let step = reconstruct_trace_path(current, &parent_f);
            if let Some(trace) = tracer {
                trace.push(step.clone());
            }
            emit(step);

            if closed_b.get(&current).is_some_and(|visited| *visited)
                && let Some(backward_cost) = g_b.get(&current)
            {
                let total = cost + *backward_cost;
                if total < best_total_cost {
                    best_total_cost = total;
                    best_meet = Some(current);
                }
            }

            for next in current.neighbours_open(maze, &maze.unit_shape) {
                let tentative = cost + maze[next] as u32;
                if g_f.get(&next).is_none_or(|best| tentative < *best) {
                    g_f.insert(next, tentative);
                    parent_f.insert(next, current);
                    open_f.push(BiAStarNode {
                        node: next,
                        cost: tentative,
                        heu_cost: tentative + heu(next, destination),
                    });
                }
            }
        }

        if let Some(node) = open_b.pop() {
            let current = node.node;
            let cost = node.cost;
            if g_b.get(&current).is_some_and(|best| cost > *best) {
                continue;
            }

            closed_b.insert(current, true);
            let step = reconstruct_trace_path(current, &parent_b);
            if let Some(trace) = tracer {
                trace.push(step.clone());
            }
            emit(step);

            if closed_f.get(&current).is_some_and(|visited| *visited)
                && let Some(forward_cost) = g_f.get(&current)
            {
                let total = cost + *forward_cost;
                if total < best_total_cost {
                    best_total_cost = total;
                    best_meet = Some(current);
                }
            }

            for next in current.neighbours_open(maze, &maze.unit_shape) {
                let tentative = cost + maze[next] as u32;
                if g_b.get(&next).is_none_or(|best| tentative < *best) {
                    g_b.insert(next, tentative);
                    parent_b.insert(next, current);
                    open_b.push(BiAStarNode {
                        node: next,
                        cost: tentative,
                        heu_cost: tentative + heu(next, source),
                    });
                }
            }
        }

        if let Some(meet) = best_meet {
            let mut path = reconstruct_path(meet, &parent_f);

            let mut tail: Vec<Node> = Vec::new();
            let mut cursor = meet;
            while let Some(next) = parent_b.get(&cursor) {
                tail.push(*next);
                cursor = *next;
            }

            path.extend(tail);
            return path;
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

        let primed_path = greedy_best_first(&maze, source, destination, manhattan_heuristic, &mut None);
        assert_eq!(primed_path.first(), Some(&source));
        assert_eq!(primed_path.last(), Some(&destination));

        let iddfs_path = iddfs(&maze, source, destination, &mut None);
        assert_eq!(iddfs_path.first(), Some(&source));
        assert_eq!(iddfs_path.last(), Some(&destination));

        let bi_bfs_path = bidirectional_bfs(&maze, source, destination, &mut None);
        assert_eq!(bi_bfs_path.first(), Some(&source));
        assert_eq!(bi_bfs_path.last(), Some(&destination));

        let astar_path = a_star(&maze, source, destination, manhattan_heuristic, &mut None);
        assert_eq!(astar_path.first(), Some(&source));
        assert_eq!(astar_path.last(), Some(&destination));

        let aldous_broder_path = aldous_broder(&maze, source, destination, &mut None);
        assert_eq!(aldous_broder_path.first(), Some(&source));
        assert_eq!(aldous_broder_path.last(), Some(&destination));

        let bi_astar_path = bidirectional_a_start(&maze, source, destination, manhattan_heuristic, &mut None);
        assert_eq!(bi_astar_path.first(), Some(&source));
        assert_eq!(bi_astar_path.last(), Some(&destination));
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

        let mut gbf_steps = 0usize;
        let mut emit_gbf = |_| gbf_steps += 1;
        let path = greedy_best_first_stream(&maze, source, destination, manhattan_heuristic, &mut emit_gbf);
        assert!(gbf_steps > 0);
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

        let mut bi_bfs_steps = 0usize;
        let mut emit_bi_bfs = |_| bi_bfs_steps += 1;
        let path = bidirectional_bfs_stream(&maze, source, destination, &mut emit_bi_bfs);
        assert!(bi_bfs_steps > 0);
        assert!(!path.is_empty());

        let mut bi_astar_steps = 0usize;
        let mut emit_bi_astar = |_| bi_astar_steps += 1;
        let path = bidirectional_a_start_stream(&maze, source, destination, manhattan_heuristic, &mut emit_bi_astar);
        assert!(bi_astar_steps > 0);
        assert!(!path.is_empty());
    }
}
