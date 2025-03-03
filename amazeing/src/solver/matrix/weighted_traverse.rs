use crate::solver::matrix::common::{reconstruct_path, validate};
use crate::solver::matrix::neighbour::{neighbours, DNode, D, L, R, U};
use crate::solver::matrix::Maze;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap, HashMap};

#[derive(Debug, Clone, Eq, PartialEq)]
struct DNodeWeighted {
    pub(crate) node: DNode,
    pub(crate) cost: u32,
}

impl PartialOrd<Self> for DNodeWeighted {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DNodeWeighted {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

fn traverse(
    maze: &Maze,
    source: DNode,
    destination: DNode,
    tracer: &mut Option<Vec<Vec<DNode>>>,
) -> Vec<DNode> {
    validate(maze, source, destination);

    let capacity = maze.rows() * maze.cols();

    let mut storage: BinaryHeap<DNodeWeighted> = BinaryHeap::with_capacity(capacity);
    let mut visited: HashMap<DNode, bool> = HashMap::with_capacity(capacity);
    let mut parent: BTreeMap<DNode, DNode> = BTreeMap::new();

    storage.push(DNodeWeighted {
        node: source,
        cost: maze[source],
    });

    while let Some(node) = storage.pop() {
        let (current, cost) = (node.node, node.cost);
        visited.insert(current, true);

        if let Some(trace) = tracer {
            trace.push(reconstruct_path(current, &parent));
        }

        if current == destination {
            let path = reconstruct_path(destination, &parent);
            return path;
        }

        for next in neighbours(maze, current, &vec![R, D, L, U]) {
            if visited.get(&next).is_none() || visited.get(&next).unwrap().clone() == false {
                parent.insert(next, current);
                storage.push(DNodeWeighted {
                    node: next,
                    cost: cost + maze[next],
                });
            }
        }
    }

    Vec::new()
}

pub fn dijkstra(
    maze: &Maze,
    start: DNode,
    end: DNode,
    tracer: &mut Option<Vec<Vec<DNode>>>,
) -> Vec<DNode> {
    traverse(maze, start, end, tracer)
}
