use crate::matrix::helper::reconstruct_path::reconstruct_path;
use crate::matrix::helper::validation::validate_node;
use crate::matrix::maze::maze::Maze;
use crate::matrix::maze::neighbour::{neighbours_block, DOWN, LEFT, RIGHT, UP};
use crate::matrix::types::{Node, Tracer};
use rand::prelude::SliceRandom;
use rand::rng;
use std::collections::BTreeMap;

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
