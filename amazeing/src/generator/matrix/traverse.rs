use crate::helper::{reconstruct_path, validate_node};
use crate::maze::matrix::neighbour::{neighbours_block, D, L, R, U};
use crate::maze::matrix::Maze;
use crate::structure::stack::Stack;
use crate::structure::structure_traits::DataStorage;
use crate::DNode;
use rand::prelude::SliceRandom;
use rand::rng;
use std::collections::BTreeMap;

pub fn random(maze: &mut Maze, source: DNode, tracer: &mut Option<Vec<Vec<DNode>>>) -> Vec<DNode> {
    validate_node(maze, source);

    let mut storage = Vec::<DNode>::new();
    let mut parent: BTreeMap<DNode, DNode> = BTreeMap::new();

    storage.push(source);

    while let Some(current) = storage.pop() {
        if let Some(trace) = tracer {
            trace.push(reconstruct_path(current, &parent));
        }

        let neighbours = neighbours_block(maze, current, Some(vec![L, R, U, D]));

        if neighbours.len() > 2 {
            maze[current] = 1;
            for next in neighbours {
                parent.insert(next, current);
                storage.push(next);
            }
        }

        storage.shuffle(&mut rng())
    }

    Vec::new()
}

pub fn dfs(maze: &mut Maze, source: DNode, tracer: &mut Option<Vec<Vec<DNode>>>) -> Vec<DNode> {
    validate_node(maze, source);

    let mut storage = Stack::<DNode>::new();
    let mut parent: BTreeMap<DNode, DNode> = BTreeMap::new();

    storage.push(source);

    while let Some(current) = storage.pop() {
        if let Some(trace) = tracer {
            trace.push(reconstruct_path(current, &parent));
        }

        let mut neighbours = neighbours_block(maze, current, Some(vec![L, R, U, D]));

        if neighbours.len() > 2 {
            neighbours.shuffle(&mut rng());
            maze[current] = 1;
            for next in neighbours {
                parent.insert(next, current);
                storage.push(next);
            }
        }
    }

    Vec::new()
}
