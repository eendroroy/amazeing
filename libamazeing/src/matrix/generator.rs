use crate::matrix::helper::{reconstruct_trace_path, validate_node};
use crate::matrix::neighbour::{neighbours_block, DOWN, LEFT, RIGHT, UP};
use crate::matrix::types::{Node, Tracer};
use crate::matrix::Maze;
use rand::prelude::SliceRandom;
use rand::rng;
use std::collections::{BTreeMap, VecDeque};

/// Generates a maze using a breadth-first search (BFS) algorithm starting from the source node.
///
/// # Arguments
///
/// * `maze` - A mutable reference to the maze structure.
/// * `source` - The starting node for the BFS.
/// * `tracer` - An optional mutable reference to a tracer for recording the path.
pub fn bfs(maze: &mut Maze, source: Vec<Node>, tracer: &mut Option<Tracer>) {
    source.iter().for_each(|source| {
        validate_node(maze, *source);
    });

    let mut storage = Vec::<Node>::new();
    let mut parent: BTreeMap<Node, Node> = BTreeMap::new();

    source.iter().for_each(|source| {
        storage.push(*source);
    });

    while let Some(current) = storage.pop() {
        let neighbours = neighbours_block(maze, current, Some(vec![LEFT, RIGHT, UP, DOWN]));

        if neighbours.len() > 2 {
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
/// Generates a maze using a depth-first search (DFS) algorithm starting from the source node.
///
/// # Arguments
///
/// * `maze` - A mutable reference to the maze structure.
/// * `source` - The starting node for the DFS.
/// * `tracer` - An optional mutable reference to a tracer for recording the path.
pub fn dfs(maze: &mut Maze, source: Vec<Node>, tracer: &mut Option<Tracer>) {
    source.iter().for_each(|source| {
        validate_node(maze, *source);
    });

    let mut parent: BTreeMap<Node, Node> = BTreeMap::new();

    let mut storages: Vec<VecDeque<Node>> = source
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
                return;
            } else if let Some(current) = storage.pop_back() {
                let mut neighbours =
                    neighbours_block(maze, current, Some(vec![LEFT, RIGHT, UP, DOWN]));
                if neighbours.len() > 2 {
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
