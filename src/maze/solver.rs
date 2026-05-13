use super::helper::{reconstruct_path, reconstruct_trace_path, validate};
use super::structure::Maze;
use super::{NodeHeuFn, Pop, Push, Trace, Tracer};
use crate::maze::node::{DNodeWeightedForward, Node};
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

        let astar_path = a_star(&maze, source, destination, manhattan_heuristic, &mut None);
        assert_eq!(astar_path.first(), Some(&source));
        assert_eq!(astar_path.last(), Some(&destination));
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

        let mut astar_steps = 0usize;
        let mut emit_astar = |_| astar_steps += 1;
        let path = a_star_stream(&maze, source, destination, manhattan_heuristic, &mut emit_astar);
        assert!(astar_steps > 0);
        assert!(!path.is_empty());
    }
}
