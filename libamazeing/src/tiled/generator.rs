use super::helper::{reconstruct_trace_path, validate_node};
use super::neighbour::neighbours_block;
use super::types::{Node, Tracer};
use super::{Maze, UnitShape};
use rand::prelude::SliceRandom;
use rand::rng;
use std::collections::{BTreeMap, VecDeque};

pub fn bfs(maze: &mut Maze, unit_shape: &UnitShape, sources: &[Node], tracer: &mut Option<Tracer>) {
    sources.iter().for_each(|source| {
        validate_node(maze, *source);
    });

    let mut storage = Vec::<Node>::new();
    let mut parent: BTreeMap<Node, Node> = BTreeMap::new();

    sources.iter().for_each(|source| {
        storage.push(*source);
    });

    while let Some(current) = storage.pop() {
        let neighbours = neighbours_block(maze, current, unit_shape);

        if neighbours.len() >= unit_shape.sides() - 1 {
            maze[current] = 1;
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

pub fn dfs(maze: &mut Maze, unit_shape: &UnitShape, sources: &[Node], tracer: &mut Option<Tracer>) {
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
                let mut neighbours = neighbours_block(maze, current, unit_shape);
                if neighbours.len() >= unit_shape.sides() - 1 {
                    neighbours.shuffle(&mut rng());
                    maze[current] = 1;
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
