use crate::maze::matrix::{dijkstra_heuristic, neighbours, reconstruct_path, validate, Maze, D, L, R, U};
use crate::structure::DNode;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap, HashMap};

#[derive(Debug, Clone, Eq, PartialEq)]
struct DNodeWeighted {
    pub(crate) node: DNode,
    pub(crate) cost: u32,
    pub(crate) heu_cost: u32,
}

impl PartialOrd<Self> for DNodeWeighted {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DNodeWeighted {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heu_cost.cmp(&self.heu_cost)
    }
}

fn weighted_traverse(
    maze: &Maze,
    source: DNode,
    destination: DNode,
    heu: fn(DNode, DNode) -> u32,
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
        heu_cost: maze[source] + heu(source, destination),
    });

    while let Some(node) = storage.pop() {
        let (current, cost, _) = (node.node, node.cost, node.heu_cost);
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
                    heu_cost: cost + maze[next] + heu(next, destination),
                });
            }
        }
    }

    if let Some(trace) = tracer {
        trace.push(vec![source, destination]);
    }

    Vec::new()
}

pub fn dijkstra(
    maze: &Maze,
    start: DNode,
    end: DNode,
    tracer: &mut Option<Vec<Vec<DNode>>>,
) -> Vec<DNode> {
    weighted_traverse(maze, start, end, dijkstra_heuristic, tracer)
}

pub fn a_star(
    maze: &Maze,
    start: DNode,
    end: DNode,
    heu: fn(DNode, DNode) -> u32,
    tracer: &mut Option<Vec<Vec<DNode>>>,
) -> Vec<DNode> {
    weighted_traverse(maze, start, end, heu, tracer)
}
