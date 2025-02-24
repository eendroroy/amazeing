use crate::square::neighbour::{neighbours, Neighbour, D, L, R, U};
use crate::square::Maze;
use crate::structure::queue::Queue;
use crate::structure::stack::Stack;
use crate::structure::structure_traits::DataStorage;
use std::collections::{BTreeMap, HashMap};

fn validate<const ROWS: usize, const COLS: usize>(
    maze: &Maze<ROWS, COLS>,
    source: (usize, usize),
    destination: (usize, usize),
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

fn traverse<const ROWS: usize, const COLS: usize>(
    maze: &Maze<ROWS, COLS>,
    source: (usize, usize),
    direction: &Vec<Neighbour>,
    destination: (usize, usize),
    storage: &mut dyn DataStorage<(usize, usize)>,
) -> Vec<(usize, usize)> {
    validate(maze, source, destination);

    let mut visited: HashMap<(usize, usize), bool> = HashMap::with_capacity(ROWS * COLS);
    let mut parent: BTreeMap<(usize, usize), (usize, usize)> = BTreeMap::new();

    storage.push(source);

    while let Some(current) = storage.pop() {
        visited.insert(current, true);

        if current == destination {
            let mut path = Vec::<(usize, usize)>::new();
            let mut current = destination;
            while parent.contains_key(&current) {
                path.push(current);
                current = parent[&current];
            }
            path.push(current);
            path.reverse();
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
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut queue = Queue::<(usize, usize)>::new();
    traverse(maze, start, &vec![R, D, L, U], end, &mut queue)
}

pub fn dfs<const ROWS: usize, const COLS: usize>(
    maze: &Maze<ROWS, COLS>,
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut queue = Stack::<(usize, usize)>::new();
    traverse(maze, start, &vec![D, R, L, U], end, &mut queue)
}
