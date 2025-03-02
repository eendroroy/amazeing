use crate::solver::matrix::neighbour::{neighbours, FnNeighbour, DNode, D, L, R, U};
use crate::solver::matrix::Maze;
use crate::structure::queue::Queue;
use crate::structure::stack::Stack;
use crate::structure::structure_traits::DataStorage;
use std::collections::{BTreeMap, HashMap};

fn validate<const ROWS: usize, const COLS: usize>(
    maze: &Maze<ROWS, COLS>,
    source: DNode,
    destination: DNode,
) {
    if source.0 >= ROWS || source.1 >= COLS || destination.0 >= ROWS || destination.1 >= COLS {
        panic!(
            "Invalid start({},{}) or end({},{}) node, available nodes (0,0 - {},{})",
            source.0,
            source.1,
            destination.0,
            destination.1,
            ROWS - 1,
            COLS - 1
        );
    }
    if maze[source] < 1 || maze[destination] < 1 {
        panic!(
            "Invalid start({},{} [{}]) or end({},{}, [{}]) node",
            source.0, source.1, maze[source], destination.0, destination.1, maze[destination]
        )
    }
}

fn reconstruct_path(destination: DNode, parent: &BTreeMap<DNode, DNode>) -> Vec<DNode> {
    let mut path = Vec::<DNode>::new();
    let mut current_node = destination;
    while parent.contains_key(&current_node) {
        path.push(current_node);
        current_node = parent[&current_node];
    }
    path.push(current_node);
    path.reverse();
    path
}

fn traverse<const ROWS: usize, const COLS: usize>(
    maze: &Maze<ROWS, COLS>,
    source: DNode,
    direction: &Vec<FnNeighbour>,
    destination: DNode,
    storage: &mut dyn DataStorage<DNode>,
    tracer: &mut Option<Vec<Vec<DNode>>>,
) -> Vec<DNode> {
    validate(maze, source, destination);

    let mut visited: HashMap<DNode, bool> = HashMap::with_capacity(ROWS * COLS);
    let mut parent: BTreeMap<DNode, DNode> = BTreeMap::new();

    storage.push(source);

    while let Some(current) = storage.pop() {
        visited.insert(current, true);

        if let Some(trace) = tracer {
            trace.push(reconstruct_path(current, &parent));
        }

        if current == destination {
            let path = reconstruct_path(destination, &parent);
            return path;
        }

        for next in neighbours(maze, current, direction) {
            if visited.get(&next).is_none() || visited.get(&next).unwrap().clone() == false {
                parent.insert(next, current);
                storage.push(next);
            }
        }
    }

    Vec::new()
}

pub fn bfs<const ROWS: usize, const COLS: usize>(
    maze: &Maze<ROWS, COLS>,
    start: DNode,
    end: DNode,
    tracer: &mut Option<Vec<Vec<DNode>>>,
) -> Vec<DNode> {
    let mut queue = Queue::<DNode>::new();
    traverse(maze, start, &vec![D, R, L, U], end, &mut queue, tracer)
}

pub fn dfs<const ROWS: usize, const COLS: usize>(
    maze: &Maze<ROWS, COLS>,
    start: DNode,
    end: DNode,
    tracer: &mut Option<Vec<Vec<DNode>>>,
) -> Vec<DNode> {
    let mut queue = Stack::<DNode>::new();
    traverse(maze, start, &vec![U, L, R, D], end, &mut queue, tracer)
}
