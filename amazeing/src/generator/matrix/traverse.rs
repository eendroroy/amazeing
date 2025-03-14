use crate::helper::{reconstruct_path, validate_node};
use crate::maze::matrix::neighbour::{neighbours_block, D, L, R, U};
use crate::maze::matrix::Maze;
use crate::structure::stack::Stack;
use crate::structure::structure_traits::DataStorage;
use crate::{Node, Tracer};
use rand::prelude::SliceRandom;
use rand::rng;
use std::collections::BTreeMap;

pub fn random(maze: &mut Maze, source: Node, tracer: &mut Option<Tracer>) {
    validate_node(maze, source);

    let mut storage = Vec::<Node>::new();
    let mut parent: BTreeMap<Node, Node> = BTreeMap::new();

    storage.push(source);

    while let Some(current) = storage.pop() {
        let neighbours = neighbours_block(maze, current, Some(vec![L, R, U, D]));

        if neighbours.len() > 2 {
            maze[current] = 1;
            if let Some(trace) = tracer {
                trace.push(reconstruct_path(current, &parent));
            }
            for next in neighbours {
                parent.insert(next, current);
                storage.push(next);
            }
        }

        storage.shuffle(&mut rng())
    }
}

pub fn dfs(maze: &mut Maze, source: Node, tracer: &mut Option<Tracer>) {
    validate_node(maze, source);

    let mut storage = Stack::<Node>::new();
    let mut parent: BTreeMap<Node, Node> = BTreeMap::new();

    storage.push(source);

    while let Some(current) = storage.pop() {
        let mut neighbours = neighbours_block(maze, current, Some(vec![L, R, U, D]));

        if neighbours.len() > 2 {
            neighbours.shuffle(&mut rng());
            maze[current] = 1;
            if let Some(trace) = tracer {
                trace.push(reconstruct_path(current, &parent));
            }
            for next in neighbours {
                parent.insert(next, current);
                storage.push(next);
            }
        }
    }
}
