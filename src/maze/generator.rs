use super::helper::{reconstruct_trace_path, validate_node};
use super::types::{Trace, Tracer};
use super::{Maze, Node, NodeHeuFn, OPEN};
use crate::maze::node::DNodeWeighted;
use rand::prelude::SliceRandom;
use rand::rng;
use std::collections::{BTreeMap, BinaryHeap, VecDeque};

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
        bfs_stream(&mut maze_stream, &[source], &mut emit);
        assert!(steps > 0);
    }
}
