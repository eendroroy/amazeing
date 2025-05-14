use super::helper::{reconstruct_trace_path, validate_node};
use super::types::Tracer;
use super::{Maze, Node, NodeHeuFn, OPEN};
use crate::core::tiled::node::DNodeWeighted;
use rand::prelude::SliceRandom;
use rand::rng;
use std::collections::{BTreeMap, BinaryHeap, VecDeque};

pub fn bfs(maze: &mut Maze, sources: &[Node], tracer: &mut Option<Tracer>) {
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

        if neighbours.len() >= &maze.unit_shape.sides() - 1 {
            maze[current] = OPEN;
            if let Some(trace) = tracer {
                trace.push(reconstruct_trace_path(current, &parent));
            }
            for next in neighbours {
                parent.insert(next, current);
                storage.push(next);
            }
        }

        storage.shuffle(&mut rng())
    }
}

pub fn dfs(maze: &mut Maze, sources: &[Node], tracer: &mut Option<Tracer>) {
    sources.iter().for_each(|source| {
        validate_node(maze, *source);
    });

    let mut parent: BTreeMap<Node, Node> = BTreeMap::new();

    let mut storages: Vec<VecDeque<Node>> = sources
        .iter()
        .map(|source| {
            let mut storage = VecDeque::<Node>::new();
            storage.push_back(*source);
            storage
        })
        .collect();

    let mut skip_idx = vec![];

    loop {
        storages.iter_mut().enumerate().for_each(|(idx, storage)| {
            if skip_idx.contains(&idx) {
            } else if let Some(current) = storage.pop_back() {
                let mut neighbours = current.neighbours_block(maze, &maze.unit_shape);
                if neighbours.len() >= &maze.unit_shape.sides() - 1 {
                    neighbours.shuffle(&mut rng());
                    maze[current] = OPEN;
                    if let Some(trace) = tracer {
                        trace.push(reconstruct_trace_path(current, &parent));
                    }
                    for next in neighbours {
                        parent.insert(next, current);
                        storage.push_back(next);
                    }
                }
            } else {
                skip_idx.push(idx);
            }
        });

        if storages.len() == skip_idx.len() {
            break;
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

        if neighbours.len() >= &maze.unit_shape.sides() - 1 {
            maze[current] = OPEN;
            if let Some(trace) = tracer {
                trace.push(reconstruct_trace_path(current, &parent));
            }
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
