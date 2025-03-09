use crate::helper::validate_node;
use crate::structure::queue::Queue;
use crate::structure::stack::Stack;
use crate::structure::structure_traits::DataStorage;
use crate::structure::DNode;
use rand::prelude::SliceRandom;
use rand::rng;
use std::collections::BTreeMap;
use crate::maze::matrix::Maze;
use crate::maze::matrix::neighbour::{neighbours_block, D, L, R, U};

fn traverse(
    maze: &mut Maze,
    source: DNode,
    storage: &mut dyn DataStorage<DNode>,
    _tracer: &mut Option<Vec<Vec<DNode>>>,
) -> Vec<DNode> {
    validate_node(maze, source);

    let mut parent: BTreeMap<DNode, DNode> = BTreeMap::new();

    storage.push(source);

    while let Some(current) = storage.pop() {
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

pub fn bfs(maze: &mut Maze, start: DNode, tracer: &mut Option<Vec<Vec<DNode>>>) -> Vec<DNode> {
    let mut queue = Queue::<DNode>::new();
    traverse(maze, start, &mut queue, tracer)
}

pub fn dfs(maze: &mut Maze, start: DNode, tracer: &mut Option<Vec<Vec<DNode>>>) -> Vec<DNode> {
    let mut queue = Stack::<DNode>::new();
    traverse(maze, start, &mut queue, tracer)
}
