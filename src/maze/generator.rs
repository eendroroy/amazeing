use super::helper::{reconstruct_trace_path, validate_node};
use super::heuristics::SCALE as HEURISTIC_SCALE;
use super::types::{Trace, Tracer};
use super::{BLOCK, Maze, Node, NodeFactory, NodeHeuFn, OPEN, VOID};
use crate::maze::node::{DNodeWeighted, DNodeWeightedForward};
use rand::prelude::SliceRandom;
use rand::rng;
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

pub fn bfs(maze: &mut Maze, sources: &[Node], tracer: &mut Option<Tracer>) {
    let needs_trace = tracer.is_some();
    let mut noop = |_| {};
    bfs_impl(maze, sources, tracer, &mut noop, needs_trace);
}

pub fn bfs_stream(maze: &mut Maze, sources: &[Node], emit: &mut dyn FnMut(Trace)) {
    let mut tracer = None;
    bfs_impl(maze, sources, &mut tracer, emit, true);
}

fn bfs_impl(
    maze: &mut Maze,
    sources: &[Node],
    tracer: &mut Option<Tracer>,
    emit: &mut dyn FnMut(Trace),
    needs_trace: bool,
) {
    sources.iter().for_each(|source| {
        validate_node(maze, *source);
    });

    // Use a Vec as a bag; random swap_remove gives O(1) random selection
    // instead of O(n) shuffle-then-pop used previously.
    let mut storage = Vec::<Node>::new();
    let mut parent: HashMap<Node, Node> = HashMap::new();

    sources.iter().for_each(|source| {
        storage.push(*source);
    });

    while !storage.is_empty() {
        let idx = rand::random_range(0..storage.len());
        let current = storage.swap_remove(idx);

        let neighbours = current.neighbours_block(maze, &maze.unit_shape);

        if neighbours.len() >= maze.unit_shape.sides(current) - 1 {
            maze[current] = OPEN;
            emit_step(current, &parent, tracer, emit, needs_trace);
            for next in neighbours {
                parent.insert(next, current);
                storage.push(next);
            }
        }
    }
}

pub fn dfs(maze: &mut Maze, sources: &[Node], tracer: &mut Option<Tracer>) {
    let needs_trace = tracer.is_some();
    let mut noop = |_| {};
    dfs_impl(maze, sources, tracer, &mut noop, needs_trace);
}

pub fn dfs_stream(maze: &mut Maze, sources: &[Node], emit: &mut dyn FnMut(Trace)) {
    let mut tracer = None;
    dfs_impl(maze, sources, &mut tracer, emit, true);
}

pub fn prim(maze: &mut Maze, sources: &[Node], tracer: &mut Option<Tracer>) {
    let needs_trace = tracer.is_some();
    let mut noop = |_| {};
    prim_impl(maze, sources, tracer, &mut noop, needs_trace);
}

pub fn prim_stream(maze: &mut Maze, sources: &[Node], emit: &mut dyn FnMut(Trace)) {
    let mut tracer = None;
    prim_impl(maze, sources, &mut tracer, emit, true);
}

fn prim_impl(
    maze: &mut Maze,
    sources: &[Node],
    tracer: &mut Option<Tracer>,
    emit: &mut dyn FnMut(Trace),
    needs_trace: bool,
) {
    if sources.is_empty() {
        return;
    }

    sources.iter().for_each(|source| {
        validate_node(maze, *source);
    });

    let mut visited: HashSet<Node> = HashSet::with_capacity(maze.rows() * maze.cols());
    let mut parent: HashMap<Node, Node> = HashMap::new();
    let mut frontier: Vec<(Node, Node)> = Vec::new();

    for source in sources {
        if visited.contains(source) {
            continue;
        }
        visited.insert(*source);
        maze[*source] = OPEN;

        emit_step(*source, &parent, tracer, emit, needs_trace);

        for n in source.neighbours_block(maze, &maze.unit_shape) {
            frontier.push((n, *source));
        }
    }

    while !frontier.is_empty() {
        let idx = rand::random_range(0..frontier.len());
        let (current, _from) = frontier.swap_remove(idx);

        if visited.contains(&current) {
            continue;
        }

        let mut connectors = current.neighbours_open(maze, &maze.unit_shape);
        if connectors.len() != 1 {
            continue;
        }

        let connector = connectors.swap_remove(0);

        parent.insert(current, connector);
        visited.insert(current);
        maze[current] = OPEN;

        emit_step(current, &parent, tracer, emit, needs_trace);

        for next in current.neighbours_block(maze, &maze.unit_shape) {
            if !visited.contains(&next) {
                frontier.push((next, current));
            }
        }
    }
}

pub fn beam_search(
    maze: &mut Maze,
    sources: &[Node],
    destination: Node,
    heu: NodeHeuFn,
    jumble_factor: u32,
    tracer: &mut Option<Tracer>,
) {
    let needs_trace = tracer.is_some();
    let mut noop = |_| {};
    beam_search_impl(maze, sources, destination, heu, jumble_factor, tracer, &mut noop, needs_trace);
}

pub fn beam_search_stream(
    maze: &mut Maze,
    sources: &[Node],
    destination: Node,
    heu: NodeHeuFn,
    jumble_factor: u32,
    emit: &mut dyn FnMut(Trace),
) {
    let mut tracer = None;
    beam_search_impl(maze, sources, destination, heu, jumble_factor, &mut tracer, emit, true);
}

#[allow(clippy::too_many_arguments)]
fn beam_search_impl(
    maze: &mut Maze,
    sources: &[Node],
    destination: Node,
    heu: NodeHeuFn,
    jumble_factor: u32,
    tracer: &mut Option<Tracer>,
    emit: &mut dyn FnMut(Trace),
    needs_trace: bool,
) {
    const BEAM_WIDTH: usize = 16;

    sources.iter().for_each(|source| validate_node(maze, *source));
    validate_node(maze, destination);

    let mut discovered: HashSet<Node> = HashSet::with_capacity(maze.rows() * maze.cols());
    let mut parent: HashMap<Node, Node> = HashMap::new();
    let mut frontier: Vec<(Node, Node)> = Vec::new();

    for source in sources {
        discovered.insert(*source);
        maze[*source] = OPEN;
        emit_step(*source, &parent, tracer, emit, needs_trace);

        for next in source.neighbours_block(maze, &maze.unit_shape) {
            frontier.push((next, *source));
        }
    }

    while !frontier.is_empty() {
        let mut candidates: Vec<(Node, Node, u32)> = frontier
            .iter()
            .filter(|(node, _)| maze[*node] == BLOCK)
            .map(|(node, from)| {
                (*node, *from, heu(*node, destination) + rand::random_range(0..=jumble_factor * HEURISTIC_SCALE))
            })
            .collect();

        if candidates.is_empty() {
            break;
        }

        candidates.shuffle(&mut rng());
        candidates.sort_by_key(|(_, _, score)| *score);
        if candidates.len() > BEAM_WIDTH {
            candidates.truncate(BEAM_WIDTH);
        }

        let mut carved_any = false;
        for (node, from, _) in candidates {
            if maze[node] != BLOCK {
                continue;
            }

            let mut connectors = node.neighbours_open(maze, &maze.unit_shape);
            if connectors.len() != 1 {
                continue;
            }

            let connector = if maze[from] == OPEN { from } else { connectors.swap_remove(0) };

            discovered.insert(node);
            parent.insert(node, connector);
            maze[node] = OPEN;
            carved_any = true;

            emit_step(node, &parent, tracer, emit, needs_trace);

            for next in node.neighbours_block(maze, &maze.unit_shape) {
                frontier.push((next, node));
            }
        }

        frontier.retain(|(node, _)| maze[*node] == BLOCK);
        if !carved_any {
            if let Some(pos) = frontier
                .iter()
                .position(|(node, _)| maze[*node] == BLOCK && node.neighbours_open(maze, &maze.unit_shape).len() == 1)
            {
                let (node, from) = frontier.swap_remove(pos);
                let mut connectors = node.neighbours_open(maze, &maze.unit_shape);
                let connector = if maze[from] == OPEN { from } else { connectors.swap_remove(0) };

                discovered.insert(node);
                parent.insert(node, connector);
                maze[node] = OPEN;

                emit_step(node, &parent, tracer, emit, needs_trace);

                for next in node.neighbours_block(maze, &maze.unit_shape) {
                    frontier.push((next, node));
                }
                frontier.retain(|(n, _)| maze[*n] == BLOCK);
                continue;
            }

            break;
        }
    }
}

pub fn bidirectional_greedy_best_first(
    maze: &mut Maze,
    sources: &[Node],
    destination: Node,
    heu: NodeHeuFn,
    jumble_factor: u32,
    tracer: &mut Option<Tracer>,
) {
    let needs_trace = tracer.is_some();
    let mut noop = |_| {};
    bidirectional_greedy_best_first_impl(
        maze,
        sources,
        destination,
        heu,
        jumble_factor,
        tracer,
        &mut noop,
        needs_trace,
    );
}

pub fn bidirectional_greedy_best_first_stream(
    maze: &mut Maze,
    sources: &[Node],
    destination: Node,
    heu: NodeHeuFn,
    jumble_factor: u32,
    emit: &mut dyn FnMut(Trace),
) {
    let mut tracer = None;
    bidirectional_greedy_best_first_impl(maze, sources, destination, heu, jumble_factor, &mut tracer, emit, true);
}

#[allow(clippy::too_many_arguments)]
fn bidirectional_greedy_best_first_impl(
    maze: &mut Maze,
    sources: &[Node],
    destination: Node,
    heu: NodeHeuFn,
    jumble_factor: u32,
    tracer: &mut Option<Tracer>,
    emit: &mut dyn FnMut(Trace),
    needs_trace: bool,
) {
    if sources.is_empty() {
        return;
    }

    sources.iter().for_each(|source| validate_node(maze, *source));
    validate_node(maze, destination);

    let mut forward: BinaryHeap<DNodeWeightedForward> = BinaryHeap::new();
    let mut backward: BinaryHeap<DNodeWeightedForward> = BinaryHeap::new();
    let mut discovered_f: HashSet<Node> = HashSet::with_capacity(maze.rows() * maze.cols());
    let mut discovered_b: HashSet<Node> = HashSet::with_capacity(maze.rows() * maze.cols());
    let mut parent_f: HashMap<Node, Node> = HashMap::new();
    let mut parent_b: HashMap<Node, Node> = HashMap::new();
    let backward_target = sources[rand::random_range(0..sources.len())];

    for source in sources {
        discovered_f.insert(*source);
        maze[*source] = OPEN;
        forward.push(DNodeWeightedForward {
            node: *source,
            cost: 0,
            heu_cost: heu(*source, destination) + rand::random_range(0..=jumble_factor * HEURISTIC_SCALE),
        });
        emit_step(*source, &parent_f, tracer, emit, needs_trace);
    }

    discovered_b.insert(destination);
    maze[destination] = OPEN;
    backward.push(DNodeWeightedForward {
        node: destination,
        cost: 0,
        heu_cost: heu(destination, backward_target) + rand::random_range(0..=jumble_factor * HEURISTIC_SCALE),
    });
    emit_step(destination, &parent_b, tracer, emit, needs_trace);

    while !forward.is_empty() || !backward.is_empty() {
        if let Some(node) = forward.pop() {
            let current = node.node;
            let mut neighbours = current.neighbours_block(maze, &maze.unit_shape);
            neighbours.shuffle(&mut rng());
            for next in neighbours {
                if discovered_f.contains(&next) {
                    continue;
                }
                if next.neighbours_open(maze, &maze.unit_shape).len() != 1 {
                    continue;
                }

                discovered_f.insert(next);
                parent_f.insert(next, current);
                maze[next] = OPEN;
                forward.push(DNodeWeightedForward {
                    node: next,
                    cost: 0,
                    heu_cost: heu(next, destination) + rand::random_range(0..=jumble_factor * HEURISTIC_SCALE),
                });
                emit_step(next, &parent_f, tracer, emit, needs_trace);
            }
        }

        if let Some(node) = backward.pop() {
            let current = node.node;
            let mut neighbours = current.neighbours_block(maze, &maze.unit_shape);
            neighbours.shuffle(&mut rng());
            for next in neighbours {
                if discovered_b.contains(&next) {
                    continue;
                }
                if next.neighbours_open(maze, &maze.unit_shape).len() != 1 {
                    continue;
                }

                discovered_b.insert(next);
                parent_b.insert(next, current);
                maze[next] = OPEN;
                backward.push(DNodeWeightedForward {
                    node: next,
                    cost: 0,
                    heu_cost: heu(next, backward_target) + rand::random_range(0..=jumble_factor * HEURISTIC_SCALE),
                });
                emit_step(next, &parent_b, tracer, emit, needs_trace);
            }
        }
    }
}

pub fn simulated_annealing_search(
    maze: &mut Maze,
    sources: &[Node],
    destination: Option<Node>,
    heu: NodeHeuFn,
    tracer: &mut Option<Tracer>,
) {
    let needs_trace = tracer.is_some();
    let mut noop = |_| {};
    simulated_annealing_search_impl(maze, sources, destination, heu, tracer, &mut noop, needs_trace);
}

pub fn simulated_annealing_search_stream(
    maze: &mut Maze,
    sources: &[Node],
    destination: Option<Node>,
    heu: NodeHeuFn,
    emit: &mut dyn FnMut(Trace),
) {
    let mut tracer = None;
    simulated_annealing_search_impl(maze, sources, destination, heu, &mut tracer, emit, true);
}

fn simulated_annealing_search_impl(
    maze: &mut Maze,
    sources: &[Node],
    destination: Option<Node>,
    heu: NodeHeuFn,
    tracer: &mut Option<Tracer>,
    emit: &mut dyn FnMut(Trace),
    needs_trace: bool,
) {
    if sources.is_empty() {
        return;
    }
    sources.iter().for_each(|source| validate_node(maze, *source));
    if let Some(dest) = destination {
        validate_node(maze, dest);
    }

    let mut parent: HashMap<Node, Node> = HashMap::new();
    let mut walkers = sources.to_vec();
    let mut temperature = (maze.rows() + maze.cols()).max(2) as f32;
    let cooling = 0.996f32;
    let max_steps = maze.rows().saturating_mul(maze.cols()).saturating_mul(256);

    for source in sources {
        maze[*source] = OPEN;
        emit_step(*source, &parent, tracer, emit, needs_trace);
    }

    for _ in 0..max_steps {
        for walker in walkers.iter_mut() {
            let options: Vec<Node> = walker
                .neighbours(&maze.unit_shape)
                .into_iter()
                .filter(|n| maze[*n] != VOID)
                .collect();
            if options.is_empty() {
                continue;
            }

            let candidate = options[rand::random_range(0..options.len())];
            let mut accept = true;
            if let Some(dest) = destination {
                let current_h = heu(*walker, dest) as f32;
                let candidate_h = heu(candidate, dest) as f32;
                let delta = candidate_h - current_h;
                accept = delta <= 0.0 || rand::random::<f32>() < (-delta / temperature.max(0.001)).exp();
            }

            if accept {
                if maze[candidate] == OPEN {
                    *walker = candidate;
                    continue;
                }

                if maze[candidate] == BLOCK && candidate.neighbours_open(maze, &maze.unit_shape).len() == 1 {
                    parent.insert(candidate, *walker);
                    maze[candidate] = OPEN;
                    emit_step(candidate, &parent, tracer, emit, needs_trace);
                    *walker = candidate;
                }
            }
        }
        temperature *= cooling;
    }

    // Finish quickly by carving any remaining valid frontier cells.
    let mut frontier: Vec<Node> = Vec::new();
    for r in 0..maze.rows() {
        for c in 0..maze.cols() {
            let node = NodeFactory::new(maze.rows(), maze.cols())
                .at(r, c)
                .expect("indices should be in-bounds");
            if maze[node] == BLOCK && node.neighbours_open(maze, &maze.unit_shape).len() == 1 {
                frontier.push(node);
            }
        }
    }

    while !frontier.is_empty() {
        let idx = rand::random_range(0..frontier.len());
        let node = frontier.swap_remove(idx);
        if maze[node] != BLOCK {
            continue;
        }

        let mut connectors = node.neighbours_open(maze, &maze.unit_shape);
        if connectors.len() != 1 {
            continue;
        }

        let connector = connectors.swap_remove(0);
        parent.insert(node, connector);
        maze[node] = OPEN;

        emit_step(node, &parent, tracer, emit, needs_trace);

        for next in node.neighbours_block(maze, &maze.unit_shape) {
            if next.neighbours_open(maze, &maze.unit_shape).len() == 1 {
                frontier.push(next);
            }
        }
    }
}

fn dfs_impl(
    maze: &mut Maze,
    sources: &[Node],
    tracer: &mut Option<Tracer>,
    emit: &mut dyn FnMut(Trace),
    needs_trace: bool,
) {
    sources.iter().for_each(|source| {
        validate_node(maze, *source);
    });

    let mut parent: HashMap<Node, Node> = HashMap::new();

    let mut storages: Vec<VecDeque<Node>> = sources
        .iter()
        .map(|source| {
            let mut storage = VecDeque::with_capacity(maze.unit_shape.sides(*source));
            storage.push_back(*source);
            storage
        })
        .collect();

    let mut skip_idx = vec![false; storages.len()];
    let mut finished = 0;

    while finished < storages.len() {
        for (idx, storage) in storages.iter_mut().enumerate() {
            if skip_idx[idx] {
                continue;
            }
            if let Some(current) = storage.pop_back() {
                let mut neighbours = current.neighbours_block(maze, &maze.unit_shape);
                if neighbours.len() >= maze.unit_shape.sides(current) - 1 {
                    neighbours.shuffle(&mut rng());
                    maze[current] = OPEN;
                    emit_step(current, &parent, tracer, emit, needs_trace);
                    for next in neighbours {
                        parent.insert(next, current);
                        storage.push_back(next);
                    }
                }
            } else {
                skip_idx[idx] = true;
                finished += 1;
            }
        }
    }
}

pub fn iddfs(maze: &mut Maze, sources: &[Node], tracer: &mut Option<Tracer>) {
    let needs_trace = tracer.is_some();
    let mut noop = |_| {};
    iddfs_impl(maze, sources, tracer, &mut noop, needs_trace);
}

pub fn iddfs_stream(maze: &mut Maze, sources: &[Node], emit: &mut dyn FnMut(Trace)) {
    let mut tracer = None;
    iddfs_impl(maze, sources, &mut tracer, emit, true);
}

fn iddfs_impl(
    maze: &mut Maze,
    sources: &[Node],
    tracer: &mut Option<Tracer>,
    emit: &mut dyn FnMut(Trace),
    needs_trace: bool,
) {
    sources.iter().for_each(|source| {
        validate_node(maze, *source);
    });

    let max_depth = maze.rows() * maze.cols();

    for depth_limit in 0..=max_depth {
        let mut visited: HashSet<Node> = HashSet::with_capacity(max_depth);
        let mut parent: HashMap<Node, Node> = HashMap::new();

        for source in sources {
            iddfs_depth_limited(maze, *source, depth_limit, &mut visited, &mut parent, tracer, emit, needs_trace);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn iddfs_depth_limited(
    maze: &mut Maze,
    source: Node,
    max_depth: usize,
    visited: &mut HashSet<Node>,
    parent: &mut HashMap<Node, Node>,
    tracer: &mut Option<Tracer>,
    emit: &mut dyn FnMut(Trace),
    needs_trace: bool,
) {
    let mut storage: Vec<(Node, usize)> = vec![(source, 0)];

    while let Some((current, depth)) = storage.pop() {
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current);

        let mut neighbours = current.neighbours_block(maze, &maze.unit_shape);
        if neighbours.len() >= maze.unit_shape.sides(current) - 1 {
            neighbours.shuffle(&mut rng());
            maze[current] = OPEN;
            emit_step(current, parent, tracer, emit, needs_trace);

            if depth < max_depth {
                for next in neighbours {
                    if !visited.contains(&next) {
                        parent.insert(next, current);
                        storage.push((next, depth + 1));
                    }
                }
            }
        }
    }
}

pub fn aldous_broder(maze: &mut Maze, sources: &[Node], tracer: &mut Option<Tracer>) {
    let needs_trace = tracer.is_some();
    let mut noop = |_| {};
    aldous_broder_impl(maze, sources, tracer, &mut noop, needs_trace);
}

pub fn aldous_broder_stream(maze: &mut Maze, sources: &[Node], emit: &mut dyn FnMut(Trace)) {
    let mut tracer = None;
    aldous_broder_impl(maze, sources, &mut tracer, emit, true);
}

fn aldous_broder_impl(
    maze: &mut Maze,
    sources: &[Node],
    tracer: &mut Option<Tracer>,
    emit: &mut dyn FnMut(Trace),
    needs_trace: bool,
) {
    if sources.is_empty() {
        return;
    }

    sources.iter().for_each(|source| validate_node(maze, *source));

    let mut remaining = 0usize;
    let factory = NodeFactory::new(maze.rows(), maze.cols());
    for r in 0..maze.rows() {
        for c in 0..maze.cols() {
            let node = factory.at(r, c).expect("indices should be in-bounds");
            if maze[node] == BLOCK {
                remaining += 1;
            }
        }
    }

    let mut visited: HashSet<Node> = HashSet::with_capacity(maze.rows() * maze.cols());
    let mut parent: HashMap<Node, Node> = HashMap::new();

    let mut walkers = sources.to_vec();
    walkers.shuffle(&mut rng());

    for source in sources {
        if visited.insert(*source) {
            if maze[*source] == BLOCK {
                maze[*source] = OPEN;
                remaining = remaining.saturating_sub(1);
            }
            emit_step(*source, &parent, tracer, emit, needs_trace);
        }
    }

    while remaining > 0 {
        for walker in walkers.iter_mut() {
            let options: Vec<Node> = walker
                .neighbours(&maze.unit_shape)
                .into_iter()
                .filter(|n| maze[*n] != VOID)
                .collect();

            if options.is_empty() {
                continue;
            }

            let next = options[rand::random_range(0..options.len())];

            if visited.insert(next) {
                parent.insert(next, *walker);
                if maze[next] == BLOCK {
                    maze[next] = OPEN;
                    remaining = remaining.saturating_sub(1);
                }
                emit_step(next, &parent, tracer, emit, needs_trace);
            }

            *walker = next;
        }
    }
}

pub fn a_star<T: DNodeWeighted>(
    maze: &mut Maze,
    sources: &[Node],
    destination: Node,
    heu: NodeHeuFn,
    jumble_factor: u32,
    tracer: &mut Option<Tracer>,
) {
    let needs_trace = tracer.is_some();
    let mut noop = |_| {};
    a_star_impl::<T>(maze, sources, destination, heu, jumble_factor, tracer, &mut noop, needs_trace);
}

pub fn a_star_stream<T: DNodeWeighted>(
    maze: &mut Maze,
    sources: &[Node],
    destination: Node,
    heu: NodeHeuFn,
    jumble_factor: u32,
    emit: &mut dyn FnMut(Trace),
) {
    let mut tracer = None;
    a_star_impl::<T>(maze, sources, destination, heu, jumble_factor, &mut tracer, emit, true);
}

pub fn bidirectional_a_start(
    maze: &mut Maze,
    sources: &[Node],
    destination: Node,
    heu: NodeHeuFn,
    jumble_factor: u32,
    tracer: &mut Option<Tracer>,
) {
    let needs_trace = tracer.is_some();
    let mut noop = |_| {};
    bidirectional_a_start_impl(maze, sources, destination, heu, jumble_factor, tracer, &mut noop, needs_trace);
}

pub fn bidirectional_a_start_stream(
    maze: &mut Maze,
    sources: &[Node],
    destination: Node,
    heu: NodeHeuFn,
    jumble_factor: u32,
    emit: &mut dyn FnMut(Trace),
) {
    let mut tracer = None;
    bidirectional_a_start_impl(maze, sources, destination, heu, jumble_factor, &mut tracer, emit, true);
}

#[allow(clippy::too_many_arguments)]
fn bidirectional_a_start_impl(
    maze: &mut Maze,
    sources: &[Node],
    destination: Node,
    heu: NodeHeuFn,
    jumble_factor: u32,
    tracer: &mut Option<Tracer>,
    emit: &mut dyn FnMut(Trace),
    needs_trace: bool,
) {
    sources.iter().for_each(|source| {
        validate_node(maze, *source);
    });
    validate_node(maze, destination);

    let mut forward: BinaryHeap<DNodeWeightedForward> = BinaryHeap::new();
    let mut backward: BinaryHeap<DNodeWeightedForward> = BinaryHeap::new();
    let mut parent_forward: HashMap<Node, Node> = HashMap::new();
    let mut parent_backward: HashMap<Node, Node> = HashMap::new();

    sources.iter().for_each(|source| {
        forward.push(DNodeWeightedForward {
            node: *source,
            cost: maze[*source] as u32,
            heu_cost: maze[*source] as u32
                + heu(*source, destination)
                + rand::random_range(0..=jumble_factor * HEURISTIC_SCALE),
        });
    });
    backward.push(DNodeWeightedForward {
        node: destination,
        cost: maze[destination] as u32,
        heu_cost: maze[destination] as u32
            + heu(destination, sources[0])
            + rand::random_range(0..=jumble_factor * HEURISTIC_SCALE),
    });

    while !forward.is_empty() || !backward.is_empty() {
        if let Some(node) = forward.pop() {
            let (current, cost) = (node.node, node.cost);
            let neighbours = current.neighbours_block(maze, &maze.unit_shape);
            if neighbours.len() >= maze.unit_shape.sides(current) - 1 {
                maze[current] = OPEN;
                emit_step(current, &parent_forward, tracer, emit, needs_trace);
                for next in neighbours {
                    parent_forward.insert(next, current);
                    forward.push(DNodeWeightedForward {
                        node: next,
                        cost: cost + maze[next] as u32,
                        heu_cost: cost
                            + maze[next] as u32
                            + heu(next, destination)
                            + rand::random_range(0..=jumble_factor * HEURISTIC_SCALE),
                    });
                }
            }
        }

        if let Some(node) = backward.pop() {
            let (current, cost) = (node.node, node.cost);
            let neighbours = current.neighbours_block(maze, &maze.unit_shape);
            if neighbours.len() >= maze.unit_shape.sides(current) - 1 {
                maze[current] = OPEN;
                emit_step(current, &parent_backward, tracer, emit, needs_trace);
                for next in neighbours {
                    parent_backward.insert(next, current);
                    backward.push(DNodeWeightedForward {
                        node: next,
                        cost: cost + maze[next] as u32,
                        heu_cost: cost
                            + maze[next] as u32
                            + heu(next, sources[0])
                            + rand::random_range(0..=jumble_factor * HEURISTIC_SCALE),
                    });
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn a_star_impl<T: DNodeWeighted>(
    maze: &mut Maze,
    sources: &[Node],
    destination: Node,
    heu: NodeHeuFn,
    jumble_factor: u32,
    tracer: &mut Option<Tracer>,
    emit: &mut dyn FnMut(Trace),
    needs_trace: bool,
) {
    sources.iter().for_each(|source| {
        validate_node(maze, *source);
    });

    validate_node(maze, destination);

    let mut storage: BinaryHeap<T> = BinaryHeap::new();
    let mut parent: HashMap<Node, Node> = HashMap::new();

    sources.iter().for_each(|source| {
        storage.push(T::new(
            *source,
            maze[*source] as u32,
            maze[*source] as u32 + heu(*source, destination) + rand::random_range(0..=jumble_factor * HEURISTIC_SCALE),
        ));
    });

    while let Some(node) = storage.pop() {
        let (current, cost, _) = (node.node(), node.cost(), node.heu_cost());

        let neighbours = current.neighbours_block(maze, &maze.unit_shape);

        if neighbours.len() >= maze.unit_shape.sides(current) - 1 {
            maze[current] = OPEN;
            emit_step(current, &parent, tracer, emit, needs_trace);
            for next in neighbours {
                parent.insert(next, current);
                storage.push(T::new(
                    next,
                    cost + maze[next] as u32,
                    cost + maze[next] as u32
                        + heu(next, destination)
                        + rand::random_range(0..=jumble_factor * HEURISTIC_SCALE),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::maze::heuristics::manhattan_heuristic;
    use crate::maze::{BLOCK, DNodeWeightedForward, NodeFactory, OPEN, UnitShape};

    #[test]
    fn bfs_and_dfs_open_source_and_collect_trace() {
        let mut maze_bfs = Maze::new(UnitShape::Square, 5, 5, BLOCK);
        let mut maze_dfs = maze_bfs.clone();
        let source = NodeFactory::new(5, 5).at(2, 2).unwrap();

        let mut trace_bfs = Some(vec![]);
        bfs(&mut maze_bfs, &[source], &mut trace_bfs);
        assert_eq!(maze_bfs[source], OPEN);
        assert!(!trace_bfs.unwrap().is_empty());

        let mut trace_dfs = Some(vec![]);
        dfs(&mut maze_dfs, &[source], &mut trace_dfs);
        assert_eq!(maze_dfs[source], OPEN);
        assert!(!trace_dfs.unwrap().is_empty());

        let mut maze_prim = Maze::new(UnitShape::Square, 5, 5, BLOCK);
        let mut trace_prim = Some(vec![]);
        prim(&mut maze_prim, &[source], &mut trace_prim);
        assert_eq!(maze_prim[source], OPEN);
        assert!(!trace_prim.unwrap().is_empty());

        let mut maze_sas = Maze::new(UnitShape::Square, 5, 5, BLOCK);
        let mut trace_sas = Some(vec![]);
        simulated_annealing_search(&mut maze_sas, &[source], None, manhattan_heuristic, &mut trace_sas);
        assert_eq!(maze_sas[source], OPEN);
        assert!(!trace_sas.unwrap().is_empty());

        let mut maze_iddfs = Maze::new(UnitShape::Square, 5, 5, BLOCK);
        let mut trace_iddfs = Some(vec![]);
        iddfs(&mut maze_iddfs, &[source], &mut trace_iddfs);
        assert_eq!(maze_iddfs[source], OPEN);
        assert!(!trace_iddfs.unwrap().is_empty());

        let mut maze_ab = Maze::new(UnitShape::Square, 5, 5, BLOCK);
        let mut trace_ab = Some(vec![]);
        aldous_broder(&mut maze_ab, &[source], &mut trace_ab);
        assert_eq!(maze_ab[source], OPEN);
        assert!(!trace_ab.unwrap().is_empty());
    }

    #[test]
    fn a_star_variants_and_stream_emit_steps() {
        let mut maze = Maze::new(UnitShape::Square, 5, 5, BLOCK);
        let f = NodeFactory::new(5, 5);
        let source = f.at(2, 2).unwrap();
        let destination = f.at(4, 4).unwrap();

        let mut trace = Some(vec![]);
        a_star::<DNodeWeightedForward>(&mut maze, &[source], destination, manhattan_heuristic, 0, &mut trace);
        assert_eq!(maze[source], OPEN);
        assert!(!trace.unwrap().is_empty());

        let mut maze_stream = Maze::new(UnitShape::Square, 5, 5, BLOCK);
        let mut steps = 0usize;
        let mut emit = |_| steps += 1;
        prim_stream(&mut maze_stream, &[source], &mut emit);
        assert!(steps > 0);

        let mut maze_bi = Maze::new(UnitShape::Square, 5, 5, BLOCK);
        let mut bi_steps = 0usize;
        let mut emit_bi = |_| bi_steps += 1;
        bidirectional_a_start_stream(&mut maze_bi, &[source], destination, manhattan_heuristic, 0, &mut emit_bi);
        assert!(bi_steps > 0);
        assert_eq!(maze_bi[source], OPEN);

        let mut maze_beam = Maze::new(UnitShape::Square, 5, 5, BLOCK);
        let mut beam_steps = 0usize;
        let mut emit_beam = |_| beam_steps += 1;
        beam_search_stream(&mut maze_beam, &[source], destination, manhattan_heuristic, 0, &mut emit_beam);
        assert!(beam_steps > 0);

        let mut maze_bi_gbf = Maze::new(UnitShape::Square, 5, 5, BLOCK);
        let mut bi_gbf_steps = 0usize;
        let mut emit_bi_gbf = |_| bi_gbf_steps += 1;
        bidirectional_greedy_best_first_stream(
            &mut maze_bi_gbf,
            &[source],
            destination,
            manhattan_heuristic,
            0,
            &mut emit_bi_gbf,
        );
        assert!(bi_gbf_steps > 0);

        let mut maze_sas_stream = Maze::new(UnitShape::Square, 5, 5, BLOCK);
        let mut sas_steps = 0usize;
        let mut emit_sas = |_| sas_steps += 1;
        simulated_annealing_search_stream(
            &mut maze_sas_stream,
            &[source],
            Some(destination),
            manhattan_heuristic,
            &mut emit_sas,
        );
        assert!(sas_steps > 0);
    }
}
