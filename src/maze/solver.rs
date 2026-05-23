use super::helper::{reconstruct_path, reconstruct_trace_path, validate};
use super::structure::Maze;
use super::{NodeHeuFn, Pop, Push, Trace, Tracer};
use crate::maze::node::{DNodeWeightedForward, Node};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

// ---------------------------------------------------------------------------
// Inline helper: build and dispatch a trace step only when needed.
// Skipping reconstruct_trace_path in non-simulation mode eliminates
// O(depth) HashMap allocations per visited node.
// ---------------------------------------------------------------------------
#[inline]
fn emit_step(
    current: Node,
    parent: &HashMap<Node, Node>,
    tracer: &mut Option<Tracer>,
    emit: &mut dyn FnMut(Trace),
    needs_trace: bool,
) {
    if needs_trace {
        let step = reconstruct_trace_path(current, parent);
        if let Some(t) = tracer {
            t.push(step.clone());
        }
        emit(step);
    }
}

fn traverse(
    maze: &Maze,
    source: Node,
    destination: Node,
    push: Push,
    pop: Pop,
    tracer: &mut Option<Tracer>,
) -> Vec<Node> {
    let needs_trace = tracer.is_some();
    let mut noop = |_| {};
    traverse_impl(maze, source, destination, push, pop, tracer, &mut noop, needs_trace)
}

fn traverse_emit(
    maze: &Maze,
    source: Node,
    destination: Node,
    push: Push,
    pop: Pop,
    tracer: &mut Option<Tracer>,
    emit: &mut dyn FnMut(Trace),
) -> Vec<Node> {
    traverse_impl(maze, source, destination, push, pop, tracer, emit, true)
}

#[allow(clippy::too_many_arguments)]
fn traverse_impl(
    maze: &Maze,
    source: Node,
    destination: Node,
    push: Push,
    pop: Pop,
    tracer: &mut Option<Tracer>,
    emit: &mut dyn FnMut(Trace),
    needs_trace: bool,
) -> Vec<Node> {
    let storage = &mut VecDeque::new();
    validate(maze, source, destination);

    // HashSet replaces HashMap<Node, bool> — saves memory and simplifies API.
    let mut visited: HashSet<Node> = HashSet::with_capacity(maze.rows() * maze.cols());
    // HashMap replaces BTreeMap — O(1) vs O(log n) per lookup/insert.
    let mut parent: HashMap<Node, Node> = HashMap::with_capacity(maze.rows() * maze.cols());

    push(storage, source);

    while let Some(current) = pop(storage) {
        if !visited.insert(current) {
            continue;
        }

        emit_step(current, &parent, tracer, emit, needs_trace);

        if current == destination {
            return reconstruct_path(destination, &parent);
        }

        for next in current.neighbours_open(maze, &maze.unit_shape) {
            if !visited.contains(&next) {
                parent.insert(next, current);
                push(storage, next);
            }
        }
    }

    Vec::new()
}

fn weighted_traverse(
    maze: &Maze,
    source: Node,
    destination: Node,
    heu: NodeHeuFn,
    tracer: &mut Option<Tracer>,
) -> Vec<Node> {
    let needs_trace = tracer.is_some();
    let mut noop = |_| {};
    weighted_traverse_impl(maze, source, destination, heu, tracer, &mut noop, needs_trace)
}

fn weighted_traverse_emit(
    maze: &Maze,
    source: Node,
    destination: Node,
    heu: NodeHeuFn,
    tracer: &mut Option<Tracer>,
    emit: &mut dyn FnMut(Trace),
) -> Vec<Node> {
    weighted_traverse_impl(maze, source, destination, heu, tracer, emit, true)
}

fn weighted_traverse_impl(
    maze: &Maze,
    source: Node,
    destination: Node,
    heu: NodeHeuFn,
    tracer: &mut Option<Tracer>,
    emit: &mut dyn FnMut(Trace),
    needs_trace: bool,
) -> Vec<Node> {
    validate(maze, source, destination);

    let capacity = maze.rows() * maze.cols();

    let mut storage: BinaryHeap<DNodeWeightedForward> = BinaryHeap::with_capacity(capacity);
    let mut visited: HashSet<Node> = HashSet::with_capacity(capacity);
    let mut parent: HashMap<Node, Node> = HashMap::with_capacity(capacity);

    storage.push(DNodeWeightedForward {
        node: source,
        cost: maze[source] as u32,
        heu_cost: maze[source] as u32 + heu(source, destination),
    });

    while let Some(node) = storage.pop() {
        let (current, cost, _) = (node.node, node.cost, node.heu_cost);
        if !visited.insert(current) {
            continue;
        }

        emit_step(current, &parent, tracer, emit, needs_trace);

        if current == destination {
            return reconstruct_path(destination, &parent);
        }

        for next in current.neighbours_open(maze, &maze.unit_shape) {
            if !visited.contains(&next) {
                parent.insert(next, current);
                storage.push(DNodeWeightedForward {
                    node: next,
                    cost: cost + maze[next] as u32,
                    heu_cost: cost + maze[next] as u32 + heu(next, destination),
                });
            }
        }
    }

    Vec::new()
}

pub fn bfs(maze: &Maze, source: Node, destination: Node, tracer: &mut Option<Tracer>) -> Vec<Node> {
    let push = |s: &mut VecDeque<Node>, n: Node| s.push_back(n);
    let pop = |s: &mut VecDeque<Node>| s.pop_front();
    traverse(maze, source, destination, push, pop, tracer)
}

pub fn bfs_stream(maze: &Maze, source: Node, destination: Node, emit: &mut dyn FnMut(Trace)) -> Vec<Node> {
    let push = |s: &mut VecDeque<Node>, n: Node| s.push_back(n);
    let pop = |s: &mut VecDeque<Node>| s.pop_front();
    let mut tracer = None;
    traverse_emit(maze, source, destination, push, pop, &mut tracer, emit)
}

pub fn dfs(maze: &Maze, source: Node, destination: Node, tracer: &mut Option<Tracer>) -> Vec<Node> {
    let push = |s: &mut VecDeque<Node>, n: Node| s.push_back(n);
    let pop = |s: &mut VecDeque<Node>| s.pop_back();

    traverse(maze, source, destination, push, pop, tracer)
}

pub fn dfs_stream(maze: &Maze, source: Node, destination: Node, emit: &mut dyn FnMut(Trace)) -> Vec<Node> {
    let push = |s: &mut VecDeque<Node>, n: Node| s.push_back(n);
    let pop = |s: &mut VecDeque<Node>| s.pop_back();
    let mut tracer = None;
    traverse_emit(maze, source, destination, push, pop, &mut tracer, emit)
}

pub fn beam_search(
    maze: &Maze,
    source: Node,
    destination: Node,
    heu: NodeHeuFn,
    tracer: &mut Option<Tracer>,
) -> Vec<Node> {
    let needs_trace = tracer.is_some();
    let mut noop = |_| {};
    beam_search_impl(maze, source, destination, heu, tracer, &mut noop, needs_trace)
}

pub fn beam_search_stream(
    maze: &Maze,
    source: Node,
    destination: Node,
    heu: NodeHeuFn,
    emit: &mut dyn FnMut(Trace),
) -> Vec<Node> {
    let mut tracer = None;
    beam_search_impl(maze, source, destination, heu, &mut tracer, emit, true)
}

fn beam_search_impl(
    maze: &Maze,
    source: Node,
    destination: Node,
    heu: NodeHeuFn,
    tracer: &mut Option<Tracer>,
    emit: &mut dyn FnMut(Trace),
    needs_trace: bool,
) -> Vec<Node> {
    validate(maze, source, destination);

    const BEAM_WIDTH: usize = 16;
    let mut current_layer = vec![source];
    let mut discovered: HashSet<Node> = HashSet::with_capacity(maze.rows() * maze.cols());
    let mut parent: HashMap<Node, Node> = HashMap::with_capacity(maze.rows() * maze.cols());

    discovered.insert(source);

    while !current_layer.is_empty() {
        let mut next_candidates: Vec<Node> = Vec::new();

        for current in current_layer {
            emit_step(current, &parent, tracer, emit, needs_trace);

            if current == destination {
                return reconstruct_path(destination, &parent);
            }

            for next in current.neighbours_open(maze, &maze.unit_shape) {
                if discovered.insert(next) {
                    parent.insert(next, current);
                    next_candidates.push(next);
                }
            }
        }

        next_candidates.sort_by_key(|n| heu(*n, destination));
        if next_candidates.len() > BEAM_WIDTH {
            next_candidates.truncate(BEAM_WIDTH);
        }
        current_layer = next_candidates;
    }

    Vec::new()
}

pub fn greedy_best_first(
    maze: &Maze,
    source: Node,
    destination: Node,
    heu: NodeHeuFn,
    tracer: &mut Option<Tracer>,
) -> Vec<Node> {
    let needs_trace = tracer.is_some();
    let mut noop = |_| {};
    greedy_best_first_impl(maze, source, destination, heu, tracer, &mut noop, needs_trace)
}

pub fn greedy_best_first_stream(
    maze: &Maze,
    source: Node,
    destination: Node,
    heu: NodeHeuFn,
    emit: &mut dyn FnMut(Trace),
) -> Vec<Node> {
    let mut tracer = None;
    greedy_best_first_impl(maze, source, destination, heu, &mut tracer, emit, true)
}

fn greedy_best_first_impl(
    maze: &Maze,
    source: Node,
    destination: Node,
    heu: NodeHeuFn,
    tracer: &mut Option<Tracer>,
    emit: &mut dyn FnMut(Trace),
    needs_trace: bool,
) -> Vec<Node> {
    validate(maze, source, destination);

    let capacity = maze.rows() * maze.cols();
    let mut storage: BinaryHeap<DNodeWeightedForward> = BinaryHeap::with_capacity(capacity);
    let mut visited: HashSet<Node> = HashSet::with_capacity(capacity);
    let mut discovered: HashSet<Node> = HashSet::with_capacity(capacity);
    let mut parent: HashMap<Node, Node> = HashMap::with_capacity(capacity);

    discovered.insert(source);
    storage.push(DNodeWeightedForward {
        node: source,
        cost: 0,
        heu_cost: heu(source, destination),
    });

    while let Some(node) = storage.pop() {
        let current = node.node;
        if !visited.insert(current) {
            continue;
        }

        emit_step(current, &parent, tracer, emit, needs_trace);

        if current == destination {
            return reconstruct_path(destination, &parent);
        }

        for next in current.neighbours_open(maze, &maze.unit_shape) {
            if discovered.insert(next) {
                parent.insert(next, current);
                storage.push(DNodeWeightedForward {
                    node: next,
                    cost: 0,
                    heu_cost: heu(next, destination),
                });
            }
        }
    }

    Vec::new()
}

pub fn bidirectional_bfs(maze: &Maze, source: Node, destination: Node, tracer: &mut Option<Tracer>) -> Vec<Node> {
    let needs_trace = tracer.is_some();
    let mut noop = |_| {};
    bidirectional_bfs_impl(maze, source, destination, tracer, &mut noop, needs_trace)
}

pub fn bidirectional_bfs_stream(
    maze: &Maze,
    source: Node,
    destination: Node,
    emit: &mut dyn FnMut(Trace),
) -> Vec<Node> {
    let mut tracer = None;
    bidirectional_bfs_impl(maze, source, destination, &mut tracer, emit, true)
}

fn bidirectional_bfs_impl(
    maze: &Maze,
    source: Node,
    destination: Node,
    tracer: &mut Option<Tracer>,
    emit: &mut dyn FnMut(Trace),
    needs_trace: bool,
) -> Vec<Node> {
    validate(maze, source, destination);

    if source == destination {
        let empty_parent: HashMap<Node, Node> = HashMap::new();
        emit_step(source, &empty_parent, tracer, emit, needs_trace);
        return vec![source];
    }

    let mut queue_f: VecDeque<Node> = VecDeque::new();
    let mut queue_b: VecDeque<Node> = VecDeque::new();

    let capacity = maze.rows() * maze.cols();
    let mut visited_f: HashSet<Node> = HashSet::with_capacity(capacity);
    let mut visited_b: HashSet<Node> = HashSet::with_capacity(capacity);
    let mut parent_f: HashMap<Node, Node> = HashMap::with_capacity(capacity);
    let mut parent_b: HashMap<Node, Node> = HashMap::with_capacity(capacity);

    queue_f.push_back(source);
    queue_b.push_back(destination);
    visited_f.insert(source);
    visited_b.insert(destination);

    emit_step(source, &parent_f, tracer, emit, needs_trace);
    emit_step(destination, &parent_b, tracer, emit, needs_trace);

    while !queue_f.is_empty() && !queue_b.is_empty() {
        let len_f = queue_f.len();
        for _ in 0..len_f {
            if let Some(current) = queue_f.pop_front() {
                for next in current.neighbours_open(maze, &maze.unit_shape) {
                    if visited_f.insert(next) {
                        parent_f.insert(next, current);
                        queue_f.push_back(next);

                        emit_step(next, &parent_f, tracer, emit, needs_trace);

                        if visited_b.contains(&next) {
                            let mut path = reconstruct_path(next, &parent_f);
                            let mut cursor = next;
                            while let Some(back_parent) = parent_b.get(&cursor) {
                                path.push(*back_parent);
                                cursor = *back_parent;
                            }
                            return path;
                        }
                    }
                }
            }
        }

        let len_b = queue_b.len();
        for _ in 0..len_b {
            if let Some(current) = queue_b.pop_front() {
                for next in current.neighbours_open(maze, &maze.unit_shape) {
                    if visited_b.insert(next) {
                        parent_b.insert(next, current);
                        queue_b.push_back(next);

                        emit_step(next, &parent_b, tracer, emit, needs_trace);

                        if visited_f.contains(&next) {
                            let mut path = reconstruct_path(next, &parent_f);
                            let mut cursor = next;
                            while let Some(back_parent) = parent_b.get(&cursor) {
                                path.push(*back_parent);
                                cursor = *back_parent;
                            }
                            return path;
                        }
                    }
                }
            }
        }
    }

    Vec::new()
}

pub fn bidirectional_greedy_best_first(
    maze: &Maze,
    source: Node,
    destination: Node,
    heu: NodeHeuFn,
    tracer: &mut Option<Tracer>,
) -> Vec<Node> {
    let needs_trace = tracer.is_some();
    let mut noop = |_| {};
    bidirectional_greedy_best_first_impl(maze, source, destination, heu, tracer, &mut noop, needs_trace)
}

pub fn bidirectional_greedy_best_first_stream(
    maze: &Maze,
    source: Node,
    destination: Node,
    heu: NodeHeuFn,
    emit: &mut dyn FnMut(Trace),
) -> Vec<Node> {
    let mut tracer = None;
    bidirectional_greedy_best_first_impl(maze, source, destination, heu, &mut tracer, emit, true)
}

fn bidirectional_greedy_best_first_impl(
    maze: &Maze,
    source: Node,
    destination: Node,
    heu: NodeHeuFn,
    tracer: &mut Option<Tracer>,
    emit: &mut dyn FnMut(Trace),
    needs_trace: bool,
) -> Vec<Node> {
    validate(maze, source, destination);

    let capacity = maze.rows() * maze.cols();
    let mut open_f: BinaryHeap<DNodeWeightedForward> = BinaryHeap::new();
    let mut open_b: BinaryHeap<DNodeWeightedForward> = BinaryHeap::new();
    let mut discovered_f: HashSet<Node> = HashSet::with_capacity(capacity);
    let mut discovered_b: HashSet<Node> = HashSet::with_capacity(capacity);
    let mut visited_f: HashSet<Node> = HashSet::with_capacity(capacity);
    let mut visited_b: HashSet<Node> = HashSet::with_capacity(capacity);
    let mut parent_f: HashMap<Node, Node> = HashMap::with_capacity(capacity);
    let mut parent_b: HashMap<Node, Node> = HashMap::with_capacity(capacity);

    discovered_f.insert(source);
    discovered_b.insert(destination);

    open_f.push(DNodeWeightedForward {
        node: source,
        cost: 0,
        heu_cost: heu(source, destination),
    });
    open_b.push(DNodeWeightedForward {
        node: destination,
        cost: 0,
        heu_cost: heu(destination, source),
    });

    while !open_f.is_empty() && !open_b.is_empty() {
        if let Some(front) = open_f.pop() {
            let current = front.node;
            if !visited_f.insert(current) {
                continue;
            }

            emit_step(current, &parent_f, tracer, emit, needs_trace);

            if visited_b.contains(&current) {
                let mut path = reconstruct_path(current, &parent_f);
                let mut cursor = current;
                while let Some(back_parent) = parent_b.get(&cursor) {
                    path.push(*back_parent);
                    cursor = *back_parent;
                }
                return path;
            }

            for next in current.neighbours_open(maze, &maze.unit_shape) {
                if discovered_f.insert(next) {
                    parent_f.insert(next, current);
                    open_f.push(DNodeWeightedForward {
                        node: next,
                        cost: 0,
                        heu_cost: heu(next, destination),
                    });
                }
            }
        }

        if let Some(back) = open_b.pop() {
            let current = back.node;
            if !visited_b.insert(current) {
                continue;
            }

            emit_step(current, &parent_b, tracer, emit, needs_trace);

            if visited_f.contains(&current) {
                let mut path = reconstruct_path(current, &parent_f);
                let mut cursor = current;
                while let Some(back_parent) = parent_b.get(&cursor) {
                    path.push(*back_parent);
                    cursor = *back_parent;
                }
                return path;
            }

            for next in current.neighbours_open(maze, &maze.unit_shape) {
                if discovered_b.insert(next) {
                    parent_b.insert(next, current);
                    open_b.push(DNodeWeightedForward {
                        node: next,
                        cost: 0,
                        heu_cost: heu(next, source),
                    });
                }
            }
        }
    }

    Vec::new()
}

pub fn simulated_annealing_search(
    maze: &Maze,
    source: Node,
    destination: Node,
    heu: NodeHeuFn,
    tracer: &mut Option<Tracer>,
) -> Vec<Node> {
    let needs_trace = tracer.is_some();
    let mut noop = |_| {};
    simulated_annealing_search_impl(maze, source, destination, heu, tracer, &mut noop, needs_trace)
}

pub fn simulated_annealing_search_stream(
    maze: &Maze,
    source: Node,
    destination: Node,
    heu: NodeHeuFn,
    emit: &mut dyn FnMut(Trace),
) -> Vec<Node> {
    let mut tracer = None;
    simulated_annealing_search_impl(maze, source, destination, heu, &mut tracer, emit, true)
}

fn simulated_annealing_search_impl(
    maze: &Maze,
    source: Node,
    destination: Node,
    heu: NodeHeuFn,
    tracer: &mut Option<Tracer>,
    emit: &mut dyn FnMut(Trace),
    needs_trace: bool,
) -> Vec<Node> {
    validate(maze, source, destination);

    let mut current = source;
    let mut parent: HashMap<Node, Node> = HashMap::with_capacity(maze.rows() * maze.cols());
    let mut discovered: HashSet<Node> = HashSet::with_capacity(maze.rows() * maze.cols());
    discovered.insert(source);

    let mut temperature = (maze.rows() + maze.cols()).max(2) as f32;
    let cooling = 0.995f32;
    let max_steps = maze.rows().saturating_mul(maze.cols()).saturating_mul(256);

    for _ in 0..max_steps {
        emit_step(current, &parent, tracer, emit, needs_trace);

        if current == destination {
            return reconstruct_path(destination, &parent);
        }

        let neighbours = current.neighbours_open(maze, &maze.unit_shape);
        if neighbours.is_empty() {
            break;
        }

        let candidate = neighbours[rand::random_range(0..neighbours.len())];
        let current_h = heu(current, destination) as f32;
        let candidate_h = heu(candidate, destination) as f32;
        let delta = candidate_h - current_h;
        let accept = delta <= 0.0 || rand::random::<f32>() < (-delta / temperature.max(0.001)).exp();

        if accept {
            if discovered.insert(candidate) {
                parent.insert(candidate, current);
            }
            current = candidate;
        }

        temperature *= cooling;
    }

    // Fallback keeps behavior deterministic for difficult random walks.
    greedy_best_first_impl(maze, source, destination, heu, tracer, emit, needs_trace)
}

#[allow(clippy::too_many_arguments)]
fn iddfs_depth_limited(
    maze: &Maze,
    source: Node,
    destination: Node,
    max_depth: usize,
    visited: &mut HashSet<Node>,
    parent: &mut HashMap<Node, Node>,
    tracer: &mut Option<Tracer>,
    emit: &mut dyn FnMut(Trace),
    needs_trace: bool,
) -> Option<Vec<Node>> {
    let mut stack: Vec<(Node, usize)> = vec![(source, 0)];
    let mut found_path: Option<Vec<Node>> = None;

    while let Some((current, depth)) = stack.pop() {
        if !visited.insert(current) {
            continue;
        }

        emit_step(current, parent, tracer, emit, needs_trace);

        if current == destination {
            found_path = Some(reconstruct_path(destination, parent));
            break;
        }

        if depth < max_depth {
            for next in current.neighbours_open(maze, &maze.unit_shape) {
                if !visited.contains(&next) {
                    parent.insert(next, current);
                    stack.push((next, depth + 1));
                }
            }
        }
    }

    found_path
}

pub fn iddfs(maze: &Maze, source: Node, destination: Node, tracer: &mut Option<Tracer>) -> Vec<Node> {
    let needs_trace = tracer.is_some();
    let mut noop = |_| {};
    iddfs_impl(maze, source, destination, tracer, &mut noop, needs_trace)
}

fn iddfs_impl(
    maze: &Maze,
    source: Node,
    destination: Node,
    tracer: &mut Option<Tracer>,
    emit: &mut dyn FnMut(Trace),
    needs_trace: bool,
) -> Vec<Node> {
    validate(maze, source, destination);

    let capacity = maze.rows() * maze.cols();
    let mut visited: HashSet<Node> = HashSet::with_capacity(capacity);
    let mut parent: HashMap<Node, Node> = HashMap::with_capacity(capacity);

    for depth_limit in 0..=(maze.rows() * maze.cols()) {
        visited.clear();
        parent.clear();

        if let Some(path) = iddfs_depth_limited(
            maze,
            source,
            destination,
            depth_limit,
            &mut visited,
            &mut parent,
            tracer,
            emit,
            needs_trace,
        ) && !path.is_empty()
        {
            return path;
        }
    }

    Vec::new()
}

pub fn iddfs_stream(maze: &Maze, source: Node, destination: Node, emit: &mut dyn FnMut(Trace)) -> Vec<Node> {
    let mut tracer = None;
    iddfs_impl(maze, source, destination, &mut tracer, emit, true)
}

pub fn aldous_broder(maze: &Maze, source: Node, destination: Node, tracer: &mut Option<Tracer>) -> Vec<Node> {
    let needs_trace = tracer.is_some();
    let mut noop = |_| {};
    aldous_broder_impl(maze, source, destination, tracer, &mut noop, needs_trace)
}

pub fn aldous_broder_stream(maze: &Maze, source: Node, destination: Node, emit: &mut dyn FnMut(Trace)) -> Vec<Node> {
    let mut tracer = None;
    aldous_broder_impl(maze, source, destination, &mut tracer, emit, true)
}

fn aldous_broder_impl(
    maze: &Maze,
    source: Node,
    destination: Node,
    tracer: &mut Option<Tracer>,
    emit: &mut dyn FnMut(Trace),
    needs_trace: bool,
) -> Vec<Node> {
    validate(maze, source, destination);

    let capacity = maze.rows() * maze.cols();
    let mut visited: HashSet<Node> = HashSet::with_capacity(capacity);
    let mut parent: HashMap<Node, Node> = HashMap::with_capacity(capacity);
    let mut current = source;

    visited.insert(source);

    emit_step(source, &parent, tracer, emit, needs_trace);

    if source == destination {
        return vec![source];
    }

    let max_steps = maze
        .rows()
        .saturating_mul(maze.cols())
        .saturating_mul(maze.rows())
        .saturating_mul(maze.cols())
        .saturating_mul(64);

    for _ in 0..max_steps {
        let options = current.neighbours_open(maze, &maze.unit_shape);
        if options.is_empty() {
            return Vec::new();
        }
        // Direct random pick: O(1) vs O(n) shuffle-then-index
        let next = options[rand::random_range(0..options.len())];

        if visited.insert(next) {
            parent.insert(next, current);
        }

        current = next;
        emit_step(current, &parent, tracer, emit, needs_trace);

        if current == destination {
            return reconstruct_path(destination, &parent);
        }
    }

    Vec::new()
}

pub fn bidirectional_a_start(
    maze: &Maze,
    source: Node,
    destination: Node,
    heu: NodeHeuFn,
    tracer: &mut Option<Tracer>,
) -> Vec<Node> {
    let needs_trace = tracer.is_some();
    let mut noop = |_| {};
    bidirectional_a_start_impl(maze, source, destination, heu, tracer, &mut noop, needs_trace)
}

pub fn bidirectional_a_start_stream(
    maze: &Maze,
    source: Node,
    destination: Node,
    heu: NodeHeuFn,
    emit: &mut dyn FnMut(Trace),
) -> Vec<Node> {
    let mut tracer = None;
    bidirectional_a_start_impl(maze, source, destination, heu, &mut tracer, emit, true)
}

fn bidirectional_a_start_impl(
    maze: &Maze,
    source: Node,
    destination: Node,
    heu: NodeHeuFn,
    tracer: &mut Option<Tracer>,
    emit: &mut dyn FnMut(Trace),
    needs_trace: bool,
) -> Vec<Node> {
    validate(maze, source, destination);

    if source == destination {
        let empty_parent: HashMap<Node, Node> = HashMap::new();
        emit_step(source, &empty_parent, tracer, emit, needs_trace);
        return vec![source];
    }

    let capacity = maze.rows() * maze.cols();
    let mut open_f: BinaryHeap<DNodeWeightedForward> = BinaryHeap::new();
    let mut open_b: BinaryHeap<DNodeWeightedForward> = BinaryHeap::new();

    let mut g_f: HashMap<Node, u32> = HashMap::with_capacity(capacity);
    let mut g_b: HashMap<Node, u32> = HashMap::with_capacity(capacity);
    let mut closed_f: HashSet<Node> = HashSet::with_capacity(capacity);
    let mut closed_b: HashSet<Node> = HashSet::with_capacity(capacity);

    let mut parent_f: HashMap<Node, Node> = HashMap::with_capacity(capacity);
    let mut parent_b: HashMap<Node, Node> = HashMap::with_capacity(capacity);

    g_f.insert(source, maze[source] as u32);
    g_b.insert(destination, maze[destination] as u32);

    open_f.push(DNodeWeightedForward {
        node: source,
        cost: maze[source] as u32,
        heu_cost: maze[source] as u32 + heu(source, destination),
    });
    open_b.push(DNodeWeightedForward {
        node: destination,
        cost: maze[destination] as u32,
        heu_cost: maze[destination] as u32 + heu(destination, source),
    });

    let mut best_meet: Option<Node> = None;
    let mut best_total_cost = u32::MAX;

    while !open_f.is_empty() && !open_b.is_empty() {
        if let Some(node) = open_f.pop() {
            let current = node.node;
            let cost = node.cost;
            if g_f.get(&current).is_some_and(|best| cost > *best) {
                continue;
            }

            closed_f.insert(current);
            emit_step(current, &parent_f, tracer, emit, needs_trace);

            if closed_b.contains(&current)
                && let Some(backward_cost) = g_b.get(&current)
            {
                let total = cost + *backward_cost;
                if total < best_total_cost {
                    best_total_cost = total;
                    best_meet = Some(current);
                }
            }

            for next in current.neighbours_open(maze, &maze.unit_shape) {
                let tentative = cost + maze[next] as u32;
                if g_f.get(&next).is_none_or(|best| tentative < *best) {
                    g_f.insert(next, tentative);
                    parent_f.insert(next, current);
                    open_f.push(DNodeWeightedForward {
                        node: next,
                        cost: tentative,
                        heu_cost: tentative + heu(next, destination),
                    });
                }
            }
        }

        if let Some(node) = open_b.pop() {
            let current = node.node;
            let cost = node.cost;
            if g_b.get(&current).is_some_and(|best| cost > *best) {
                continue;
            }

            closed_b.insert(current);
            emit_step(current, &parent_b, tracer, emit, needs_trace);

            if closed_f.contains(&current)
                && let Some(forward_cost) = g_f.get(&current)
            {
                let total = cost + *forward_cost;
                if total < best_total_cost {
                    best_total_cost = total;
                    best_meet = Some(current);
                }
            }

            for next in current.neighbours_open(maze, &maze.unit_shape) {
                let tentative = cost + maze[next] as u32;
                if g_b.get(&next).is_none_or(|best| tentative < *best) {
                    g_b.insert(next, tentative);
                    parent_b.insert(next, current);
                    open_b.push(DNodeWeightedForward {
                        node: next,
                        cost: tentative,
                        heu_cost: tentative + heu(next, source),
                    });
                }
            }
        }

        if let Some(meet) = best_meet {
            let mut path = reconstruct_path(meet, &parent_f);

            let mut tail: Vec<Node> = Vec::new();
            let mut cursor = meet;
            while let Some(next) = parent_b.get(&cursor) {
                tail.push(*next);
                cursor = *next;
            }

            path.extend(tail);
            return path;
        }
    }

    Vec::new()
}

pub fn a_star(maze: &Maze, source: Node, destination: Node, heu: NodeHeuFn, tracer: &mut Option<Tracer>) -> Vec<Node> {
    weighted_traverse(maze, source, destination, heu, tracer)
}

pub fn a_star_stream(
    maze: &Maze,
    source: Node,
    destination: Node,
    heu: NodeHeuFn,
    emit: &mut dyn FnMut(Trace),
) -> Vec<Node> {
    let mut tracer = None;
    weighted_traverse_emit(maze, source, destination, heu, &mut tracer, emit)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::maze::heuristics::manhattan_heuristic;
    use crate::maze::{NodeFactory, OPEN, UnitShape};

    fn open_grid(rows: usize, cols: usize) -> Maze {
        Maze::new(UnitShape::Square, rows, cols, OPEN)
    }

    #[test]
    fn solver_finds_path_for_all_algorithms() {
        let maze = open_grid(4, 4);
        let f = NodeFactory::new(4, 4);
        let source = f.at(0, 0).unwrap();
        let destination = f.at(3, 3).unwrap();

        let bfs_path = bfs(&maze, source, destination, &mut None);
        assert_eq!(bfs_path.first(), Some(&source));
        assert_eq!(bfs_path.last(), Some(&destination));

        let dfs_path = dfs(&maze, source, destination, &mut None);
        assert_eq!(dfs_path.first(), Some(&source));
        assert_eq!(dfs_path.last(), Some(&destination));

        let beam_path = beam_search(&maze, source, destination, manhattan_heuristic, &mut None);
        assert_eq!(beam_path.first(), Some(&source));
        assert_eq!(beam_path.last(), Some(&destination));

        let primed_path = greedy_best_first(&maze, source, destination, manhattan_heuristic, &mut None);
        assert_eq!(primed_path.first(), Some(&source));
        assert_eq!(primed_path.last(), Some(&destination));

        let iddfs_path = iddfs(&maze, source, destination, &mut None);
        assert_eq!(iddfs_path.first(), Some(&source));
        assert_eq!(iddfs_path.last(), Some(&destination));

        let bi_bfs_path = bidirectional_bfs(&maze, source, destination, &mut None);
        assert_eq!(bi_bfs_path.first(), Some(&source));
        assert_eq!(bi_bfs_path.last(), Some(&destination));

        let bi_gbf_path = bidirectional_greedy_best_first(&maze, source, destination, manhattan_heuristic, &mut None);
        assert_eq!(bi_gbf_path.first(), Some(&source));
        assert_eq!(bi_gbf_path.last(), Some(&destination));

        let astar_path = a_star(&maze, source, destination, manhattan_heuristic, &mut None);
        assert_eq!(astar_path.first(), Some(&source));
        assert_eq!(astar_path.last(), Some(&destination));

        let aldous_broder_path = aldous_broder(&maze, source, destination, &mut None);
        assert_eq!(aldous_broder_path.first(), Some(&source));
        assert_eq!(aldous_broder_path.last(), Some(&destination));

        let sas_path = simulated_annealing_search(&maze, source, destination, manhattan_heuristic, &mut None);
        assert_eq!(sas_path.first(), Some(&source));
        assert_eq!(sas_path.last(), Some(&destination));

        let bi_astar_path = bidirectional_a_start(&maze, source, destination, manhattan_heuristic, &mut None);
        assert_eq!(bi_astar_path.first(), Some(&source));
        assert_eq!(bi_astar_path.last(), Some(&destination));
    }

    #[test]
    fn stream_variants_emit_trace_steps() {
        let maze = open_grid(3, 3);
        let f = NodeFactory::new(3, 3);
        let source = f.at(0, 0).unwrap();
        let destination = f.at(2, 2).unwrap();

        let mut bfs_steps = 0usize;
        let mut emit_bfs = |_| bfs_steps += 1;
        let path = bfs_stream(&maze, source, destination, &mut emit_bfs);
        assert!(bfs_steps > 0);
        assert!(!path.is_empty());

        let mut dfs_steps = 0usize;
        let mut emit_dfs = |_| dfs_steps += 1;
        let path = dfs_stream(&maze, source, destination, &mut emit_dfs);
        assert!(dfs_steps > 0);
        assert!(!path.is_empty());

        let mut gbf_steps = 0usize;
        let mut emit_gbf = |_| gbf_steps += 1;
        let path = greedy_best_first_stream(&maze, source, destination, manhattan_heuristic, &mut emit_gbf);
        assert!(gbf_steps > 0);
        assert!(!path.is_empty());

        let mut beam_steps = 0usize;
        let mut emit_beam = |_| beam_steps += 1;
        let path = beam_search_stream(&maze, source, destination, manhattan_heuristic, &mut emit_beam);
        assert!(beam_steps > 0);
        assert!(!path.is_empty());

        let mut iddfs_steps = 0usize;
        let mut emit_iddfs = |_| iddfs_steps += 1;
        let path = iddfs_stream(&maze, source, destination, &mut emit_iddfs);
        assert!(iddfs_steps > 0);
        assert!(!path.is_empty());

        let mut astar_steps = 0usize;
        let mut emit_astar = |_| astar_steps += 1;
        let path = a_star_stream(&maze, source, destination, manhattan_heuristic, &mut emit_astar);
        assert!(astar_steps > 0);
        assert!(!path.is_empty());

        let mut aldous_broder_steps = 0usize;
        let mut emit_aldous_broder = |_| aldous_broder_steps += 1;
        let path = aldous_broder_stream(&maze, source, destination, &mut emit_aldous_broder);
        assert!(aldous_broder_steps > 0);
        assert!(!path.is_empty());

        let mut bi_bfs_steps = 0usize;
        let mut emit_bi_bfs = |_| bi_bfs_steps += 1;
        let path = bidirectional_bfs_stream(&maze, source, destination, &mut emit_bi_bfs);
        assert!(bi_bfs_steps > 0);
        assert!(!path.is_empty());

        let mut bi_gbf_steps = 0usize;
        let mut emit_bi_gbf = |_| bi_gbf_steps += 1;
        let path =
            bidirectional_greedy_best_first_stream(&maze, source, destination, manhattan_heuristic, &mut emit_bi_gbf);
        assert!(bi_gbf_steps > 0);
        assert!(!path.is_empty());

        let mut bi_astar_steps = 0usize;
        let mut emit_bi_astar = |_| bi_astar_steps += 1;
        let path = bidirectional_a_start_stream(&maze, source, destination, manhattan_heuristic, &mut emit_bi_astar);
        assert!(bi_astar_steps > 0);
        assert!(!path.is_empty());

        let mut sas_steps = 0usize;
        let mut emit_sas = |_| sas_steps += 1;
        let path = simulated_annealing_search_stream(&maze, source, destination, manhattan_heuristic, &mut emit_sas);
        assert!(sas_steps > 0);
        assert!(!path.is_empty());
    }
}
