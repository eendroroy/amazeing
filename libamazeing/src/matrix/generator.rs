use super::helper::{reconstruct_trace_path, validate_node};
use super::neighbour::neighbours_block;
use super::types::{Node, Tracer};
use super::{Maze, UnitShape};
use rand::prelude::SliceRandom;
use rand::rng;
use std::collections::{BTreeMap, VecDeque};

/// Generates a maze using a breadth-first search (BFS) algorithm starting from the source node.
///
/// This algorithm explores the maze space by visiting nodes in breadth-first order, creating
/// passages between nodes when they satisfy the unit shape's criteria. The result is a maze
/// with a more "spread out" pattern compared to DFS.
///
/// # Arguments
///
/// * `maze` - A mutable reference to the maze structure.
/// * `unit_shape` - The shape type that defines the neighbors structure and validation criteria.
/// * `source` - The starting nodes for the BFS algorithm.
/// * `tracer` - An optional mutable reference to a tracer for recording the path generation.
///
/// # Note
///
/// The algorithm will validate all source nodes before beginning the maze generation process.
pub fn bfs(maze: &mut Maze, unit_shape: &UnitShape, source: Vec<Node>, tracer: &mut Option<Tracer>) {
    source.iter().for_each(|source| {
        validate_node(maze, *source);
    });

    let mut storage = Vec::<Node>::new();
    let mut parent: BTreeMap<Node, Node> = BTreeMap::new();

    source.iter().for_each(|source| {
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

/// Generates a maze using a depth-first search (DFS) algorithm starting from the source nodes.
///
/// This algorithm explores the maze space by visiting nodes in depth-first order, creating
/// passages between nodes when they satisfy the unit shape's criteria. The result is a maze
/// with longer corridors and fewer branches compared to BFS.
///
/// # Arguments
///
/// * `maze` - A mutable reference to the maze structure.
/// * `unit_shape` - The shape type that defines the neighbors structure and validation criteria.
/// * `source` - The starting nodes for the DFS algorithm.
/// * `tracer` - An optional mutable reference to a tracer for recording the path generation.
///
/// # Note
///
/// The algorithm will validate all source nodes before beginning the maze generation process.
pub fn dfs(maze: &mut Maze, unit_shape: &UnitShape, source: Vec<Node>, tracer: &mut Option<Tracer>) {
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
