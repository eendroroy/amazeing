use super::helper::{reconstruct_trace_path, validate_node};
use super::types::{Trace, Tracer};
use super::{BLOCK, Maze, Node, NodeFactory, NodeHeuFn, OPEN, VOID};
use crate::maze::node::DNodeWeighted;
use rand::prelude::SliceRandom;
use rand::rng;
use std::collections::{BTreeMap, BinaryHeap, HashMap, VecDeque};

pub fn bfs(maze: &mut Maze, sources: &[Node], tracer: &mut Option<Tracer>) {
    let mut noop = |_| {};
    bfs_emit(maze, sources, tracer, &mut noop);
}

pub fn bfs_stream(maze: &mut Maze, sources: &[Node], emit: &mut dyn FnMut(Trace)) {
    let mut tracer = None;
    bfs_emit(maze, sources, &mut tracer, emit);
}

fn bfs_emit(maze: &mut Maze, sources: &[Node], tracer: &mut Option<Tracer>, emit: &mut dyn FnMut(Trace)) {
    sources.iter().for_each(|source| {
        validate_node(maze, *source);
    });

    let mut storage = Vec::<Node>::new();
    let mut parent: BTreeMap<Node, Node> = BTreeMap::new();

    sources.iter().for_each(|source| {
        storage.push(*source);
    });

    while let Some(current) = storage.pop() {
        let neighbours = current.neighbours_block(maze, &maze.unit_shape);

        if neighbours.len() >= &maze.unit_shape.sides(current) - 1 {
            maze[current] = OPEN;
            let step = reconstruct_trace_path(current, &parent);
            if let Some(trace) = tracer {
                trace.push(step.clone());
            }
            emit(step);
            for next in neighbours {
                parent.insert(next, current);
                storage.push(next);
            }
        }

        storage.shuffle(&mut rng())
    }
}

pub fn dfs(maze: &mut Maze, sources: &[Node], tracer: &mut Option<Tracer>) {
    let mut noop = |_| {};
    dfs_emit(maze, sources, tracer, &mut noop);
}

pub fn dfs_stream(maze: &mut Maze, sources: &[Node], emit: &mut dyn FnMut(Trace)) {
    let mut tracer = None;
    dfs_emit(maze, sources, &mut tracer, emit);
}

fn dfs_emit(maze: &mut Maze, sources: &[Node], tracer: &mut Option<Tracer>, emit: &mut dyn FnMut(Trace)) {
    sources.iter().for_each(|source| {
        validate_node(maze, *source);
    });

    let mut parent: BTreeMap<Node, Node> = BTreeMap::new();

    let mut storages: Vec<VecDeque<Node>> = sources
        .iter()
        .map(|source| {
            let mut storage = VecDeque::with_capacity(maze.unit_shape.sides(*source));
            storage.push_back(*source);
            storage
        })
        .collect();

    let mut skip_idx = vec![false; storages.len()];
    let mut finished = 0;

    while finished < storages.len() {
        for (idx, storage) in storages.iter_mut().enumerate() {
            if skip_idx[idx] {
                continue;
            }
            if let Some(current) = storage.pop_back() {
                let mut neighbours = current.neighbours_block(maze, &maze.unit_shape);
                if neighbours.len() >= maze.unit_shape.sides(current) - 1 {
                    neighbours.shuffle(&mut rng());
                    maze[current] = OPEN;
                    let step = reconstruct_trace_path(current, &parent);
                    if let Some(trace) = tracer {
                        trace.push(step.clone());
                    }
                    emit(step);
                    for next in neighbours {
                        parent.insert(next, current);
                        storage.push_back(next);
                    }
                }
            } else {
                skip_idx[idx] = true;
                finished += 1;
            }
        }
    }
}

pub fn iddfs(maze: &mut Maze, sources: &[Node], tracer: &mut Option<Tracer>) {
    let mut noop = |_| {};
    iddfs_emit(maze, sources, tracer, &mut noop);
}

pub fn iddfs_stream(maze: &mut Maze, sources: &[Node], emit: &mut dyn FnMut(Trace)) {
    let mut tracer = None;
    iddfs_emit(maze, sources, &mut tracer, emit);
}

fn iddfs_emit(maze: &mut Maze, sources: &[Node], tracer: &mut Option<Tracer>, emit: &mut dyn FnMut(Trace)) {
    sources.iter().for_each(|source| {
        validate_node(maze, *source);
    });

    let max_depth = maze.rows() * maze.cols();

    // Re-run depth-limited DFS with increasing limits to perform iterative deepening.
    for depth_limit in 0..=max_depth {
        let mut visited: HashMap<Node, bool> = HashMap::with_capacity(max_depth);
        let mut parent: BTreeMap<Node, Node> = BTreeMap::new();

        for source in sources {
            iddfs_depth_limited(maze, *source, depth_limit, &mut visited, &mut parent, tracer, emit);
        }
    }
}

fn iddfs_depth_limited(
    maze: &mut Maze,
    source: Node,
    max_depth: usize,
    visited: &mut HashMap<Node, bool>,
    parent: &mut BTreeMap<Node, Node>,
    tracer: &mut Option<Tracer>,
    emit: &mut dyn FnMut(Trace),
) {
    let mut storage: Vec<(Node, usize)> = vec![(source, 0)];

    while let Some((current, depth)) = storage.pop() {
        if visited.contains_key(&current) && *visited.get(&current).unwrap() {
            continue;
        }
        visited.insert(current, true);

        let mut neighbours = current.neighbours_block(maze, &maze.unit_shape);
        if neighbours.len() >= maze.unit_shape.sides(current) - 1 {
            neighbours.shuffle(&mut rng());
            maze[current] = OPEN;
            let step = reconstruct_trace_path(current, parent);
            if let Some(trace) = tracer {
                trace.push(step.clone());
            }
            emit(step);

            if depth < max_depth {
                for next in neighbours {
                    if !visited.contains_key(&next) || !(*visited.get(&next).unwrap()) {
                        parent.insert(next, current);
                        storage.push((next, depth + 1));
                    }
                }
            }
        }
    }
}

pub fn aldous_broder(maze: &mut Maze, sources: &[Node], tracer: &mut Option<Tracer>) {
    let mut noop = |_| {};
    aldous_broder_emit(maze, sources, tracer, &mut noop);
}

pub fn aldous_broder_stream(maze: &mut Maze, sources: &[Node], emit: &mut dyn FnMut(Trace)) {
    let mut tracer = None;
    aldous_broder_emit(maze, sources, &mut tracer, emit);
}

fn aldous_broder_emit(maze: &mut Maze, sources: &[Node], tracer: &mut Option<Tracer>, emit: &mut dyn FnMut(Trace)) {
    if sources.is_empty() {
        return;
    }

    sources.iter().for_each(|source| validate_node(maze, *source));

    let mut remaining = 0usize;
    let factory = NodeFactory::new(maze.rows(), maze.cols());
    for r in 0..maze.rows() {
        for c in 0..maze.cols() {
            let node = factory.at(r, c).expect("indices should be in-bounds");
            if maze[node] == BLOCK {
                remaining += 1;
            }
        }
    }

    let mut visited: HashMap<Node, bool> = HashMap::with_capacity(maze.rows() * maze.cols());
    let mut parent: BTreeMap<Node, Node> = BTreeMap::new();

    let mut walkers = sources.to_vec();
    walkers.shuffle(&mut rng());

    for source in sources {
        if !visited.contains_key(source) || !(*visited.get(source).unwrap()) {
            visited.insert(*source, true);
            if maze[*source] == BLOCK {
                maze[*source] = OPEN;
                remaining = remaining.saturating_sub(1);
            }
            let step = reconstruct_trace_path(*source, &parent);
            if let Some(trace) = tracer {
                trace.push(step.clone());
            }
            emit(step);
        }
    }

    while remaining > 0 {
        for walker in walkers.iter_mut() {
            let options: Vec<Node> = walker
                .neighbours(&maze.unit_shape)
                .into_iter()
                .filter(|n| maze[*n] != VOID)
                .collect();

            if options.is_empty() {
                continue;
            }

            let next = options[rand::random_range(0..options.len())];

            if !visited.contains_key(&next) || !(*visited.get(&next).unwrap()) {
                parent.insert(next, *walker);
                visited.insert(next, true);
                if maze[next] == BLOCK {
                    maze[next] = OPEN;
                    remaining = remaining.saturating_sub(1);
                }
                let step = reconstruct_trace_path(next, &parent);
                if let Some(trace) = tracer {
                    trace.push(step.clone());
                }
                emit(step);
            }

            *walker = next;
        }
    }
}

pub fn a_star<T: DNodeWeighted>(
    maze: &mut Maze,
    sources: &[Node],
    destination: Node,
    heu: NodeHeuFn,
    jumble_factor: u32,
    tracer: &mut Option<Tracer>,
) {
    let mut noop = |_| {};
    a_star_emit::<T>(maze, sources, destination, heu, jumble_factor, tracer, &mut noop);
}

pub fn a_star_stream<T: DNodeWeighted>(
    maze: &mut Maze,
    sources: &[Node],
    destination: Node,
    heu: NodeHeuFn,
    jumble_factor: u32,
    emit: &mut dyn FnMut(Trace),
) {
    let mut tracer = None;
    a_star_emit::<T>(maze, sources, destination, heu, jumble_factor, &mut tracer, emit);
}

fn a_star_emit<T: DNodeWeighted>(
    maze: &mut Maze,
    sources: &[Node],
    destination: Node,
    heu: NodeHeuFn,
    jumble_factor: u32,
    tracer: &mut Option<Tracer>,
    emit: &mut dyn FnMut(Trace),
) {
    sources.iter().for_each(|source| {
        validate_node(maze, *source);
    });

    validate_node(maze, destination);

    let mut storage: BinaryHeap<T> = BinaryHeap::new();
    let mut parent: BTreeMap<Node, Node> = BTreeMap::new();

    sources.iter().for_each(|source| {
        storage.push(T::new(
            *source,
            maze[*source] as u32,
            maze[*source] as u32 + heu(*source, destination) + rand::random_range(0..=jumble_factor),
        ));
    });

    while let Some(node) = storage.pop() {
        let (current, cost, _) = (node.node(), node.cost(), node.heu_cost());

        let neighbours = current.neighbours_block(maze, &maze.unit_shape);

        if neighbours.len() >= &maze.unit_shape.sides(current) - 1 {
            maze[current] = OPEN;
            let step = reconstruct_trace_path(current, &parent);
            if let Some(trace) = tracer {
                trace.push(step.clone());
            }
            emit(step);
            for next in neighbours {
                parent.insert(next, current);
                storage.push(T::new(
                    next,
                    cost + maze[next] as u32,
                    cost + maze[next] as u32 + heu(next, destination) + rand::random_range(0..=jumble_factor),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::maze::heuristics::manhattan_heuristic;
    use crate::maze::{BLOCK, DNodeWeightedForward, NodeFactory, UnitShape};

    #[test]
    fn bfs_and_dfs_open_source_and_collect_trace() {
        let mut maze_bfs = Maze::new(UnitShape::Square, 5, 5, BLOCK);
        let mut maze_dfs = maze_bfs.clone();
        let source = NodeFactory::new(5, 5).at(2, 2).unwrap();

        let mut trace_bfs = Some(vec![]);
        bfs(&mut maze_bfs, &[source], &mut trace_bfs);
        assert_eq!(maze_bfs[source], OPEN);
        assert!(!trace_bfs.unwrap().is_empty());

        let mut trace_dfs = Some(vec![]);
        dfs(&mut maze_dfs, &[source], &mut trace_dfs);
        assert_eq!(maze_dfs[source], OPEN);
        assert!(!trace_dfs.unwrap().is_empty());

        let mut maze_iddfs = Maze::new(UnitShape::Square, 5, 5, BLOCK);
        let mut trace_iddfs = Some(vec![]);
        iddfs(&mut maze_iddfs, &[source], &mut trace_iddfs);
        assert_eq!(maze_iddfs[source], OPEN);
        assert!(!trace_iddfs.unwrap().is_empty());

        let mut maze_ab = Maze::new(UnitShape::Square, 5, 5, BLOCK);
        let mut trace_ab = Some(vec![]);
        aldous_broder(&mut maze_ab, &[source], &mut trace_ab);
        assert_eq!(maze_ab[source], OPEN);
        assert!(!trace_ab.unwrap().is_empty());
    }

    #[test]
    fn a_star_variants_and_stream_emit_steps() {
        let mut maze = Maze::new(UnitShape::Square, 5, 5, BLOCK);
        let f = NodeFactory::new(5, 5);
        let source = f.at(2, 2).unwrap();
        let destination = f.at(4, 4).unwrap();

        let mut trace = Some(vec![]);
        a_star::<DNodeWeightedForward>(&mut maze, &[source], destination, manhattan_heuristic, 0, &mut trace);
        assert_eq!(maze[source], OPEN);
        assert!(!trace.unwrap().is_empty());

        let mut maze_stream = Maze::new(UnitShape::Square, 5, 5, BLOCK);
        let mut steps = 0usize;
        let mut emit = |_| steps += 1;
        aldous_broder_stream(&mut maze_stream, &[source], &mut emit);
        assert!(steps > 0);
    }
}
