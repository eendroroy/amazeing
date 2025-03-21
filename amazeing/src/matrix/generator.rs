use crate::matrix::helper::reconstruct_path;
use crate::matrix::helper::validate_node;
use crate::matrix::types::{Node, Tracer};
use crate::matrix::Maze;
use rand::prelude::SliceRandom;
use rand::rng;
use std::collections::{BTreeMap, VecDeque};
use crate::matrix::neighbour::{neighbours_block, DOWN, LEFT, RIGHT, UP};

pub fn bfs(maze: &mut Maze, source: Node, tracer: &mut Option<Tracer>) {
    validate_node(maze, source);

    let mut storage = Vec::<Node>::new();
    let mut parent: BTreeMap<Node, Node> = BTreeMap::new();

    storage.push(source);

    while let Some(current) = storage.pop() {
        let neighbours = neighbours_block(maze, current, Some(vec![LEFT, RIGHT, UP, DOWN]));

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

    let mut storage = VecDeque::<Node>::new();
    let mut parent: BTreeMap<Node, Node> = BTreeMap::new();

    storage.push_back(source);

    while let Some(current) = storage.pop_back() {
        let mut neighbours = neighbours_block(maze, current, Some(vec![LEFT, RIGHT, UP, DOWN]));

        if neighbours.len() > 2 {
            neighbours.shuffle(&mut rng());
            maze[current] = 1;
            if let Some(trace) = tracer {
                trace.push(reconstruct_path(current, &parent));
            }
            for next in neighbours {
                parent.insert(next, current);
                storage.push_back(next);
            }
        }
    }
}
