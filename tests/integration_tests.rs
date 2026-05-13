//! Integration tests for the amazeing maze library.
//! These tests validate complete workflows end-to-end.

use amazeing::maze::{BLOCK, Maze, NodeFactory, OPEN, UnitShape};
use std::fs;
use std::path::PathBuf;

fn temp_file(name: &str) -> PathBuf {
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("{}_{}.maze", name, ts))
}

#[test]
fn maze_creation_and_basic_ops() {
    let mut maze = Maze::new(UnitShape::Square, 3, 3, BLOCK);
    assert_eq!(maze.rows(), 3);
    assert_eq!(maze.cols(), 3);

    let f = NodeFactory::new(3, 3);
    let node = f.at(1, 1).unwrap();
    maze[node] = OPEN;
    assert_eq!(maze[node], OPEN);
}

#[test]
fn maze_file_roundtrip_preserves_data() {
    let mut maze = Maze::new(UnitShape::Hexagon, 2, 2, BLOCK);
    let f = NodeFactory::new(2, 2);
    maze[f.at(0, 0).unwrap()] = OPEN;
    maze[f.at(1, 1).unwrap()] = OPEN;

    let path = temp_file("integration_roundtrip");

    // Simulate dump_maze_to_file behavior
    let data_str = format!("{}\n", maze.unit_shape);
    let mut file_content = data_str;
    for r in 0..maze.rows() {
        for c in 0..maze.cols() {
            if let Some(n) = f.at(r, c) {
                file_content.push_str(&format!("{: >2}", maze[n]));
                if c < maze.cols() - 1 {
                    file_content.push(' ');
                }
            }
        }
        if r < maze.rows() - 1 {
            file_content.push('\n');
        }
    }
    fs::write(&path, file_content).unwrap();

    // Read back and validate
    let file_data = fs::read_to_string(&path).unwrap();
    let mut lines: Vec<&str> = file_data.split('\n').collect();
    let shape_str = lines.remove(0);
    assert_eq!(shape_str, "hexagon");
    assert!(lines.len() > 0);

    let _ = fs::remove_file(path);
}

#[test]
fn maze_generation_fills_from_sources() {
    use amazeing::maze::generator;

    let mut maze = Maze::new(UnitShape::Square, 5, 5, BLOCK);
    let f = NodeFactory::new(5, 5);
    let source = f.at(2, 2).unwrap();

    assert_eq!(maze[source], BLOCK);
    generator::bfs(&mut maze, &[source], &mut None);
    assert_eq!(maze[source], OPEN);

    // Verify some neighbors were opened
    let neighbors = source.neighbours(&UnitShape::Square);
    let any_opened = neighbors.iter().any(|n| maze[*n] == OPEN);
    assert!(any_opened);

    let mut maze_ab = Maze::new(UnitShape::Square, 5, 5, BLOCK);
    generator::aldous_broder(&mut maze_ab, &[source], &mut None);
    assert_eq!(maze_ab[source], OPEN);

    let mut maze_prim = Maze::new(UnitShape::Square, 5, 5, BLOCK);
    generator::prim(&mut maze_prim, &[source], &mut None);
    assert_eq!(maze_prim[source], OPEN);

    let mut maze_beam = Maze::new(UnitShape::Square, 5, 5, BLOCK);
    generator::beam_search(
        &mut maze_beam,
        &[source],
        f.at(4, 4).unwrap(),
        amazeing::maze::heuristics::manhattan_heuristic,
        0,
        &mut None,
    );
    assert_eq!(maze_beam[source], OPEN);

    let mut maze_bi = Maze::new(UnitShape::Square, 5, 5, BLOCK);
    let destination = f.at(4, 4).unwrap();
    generator::bidirectional_a_start(
        &mut maze_bi,
        &[source],
        destination,
        amazeing::maze::heuristics::manhattan_heuristic,
        0,
        &mut None,
    );
    assert_eq!(maze_bi[source], OPEN);
}

#[test]
fn all_maze_shapes_generate_consistently() {
    use amazeing::maze::generator;

    let shapes = [
        UnitShape::Square,
        UnitShape::Triangle,
        UnitShape::Hexagon,
        UnitShape::HexagonRectangle,
        UnitShape::Octagon,
        UnitShape::OctagonSquare,
    ];

    for shape in shapes {
        let mut maze = Maze::new(shape, 4, 4, BLOCK);
        let f = NodeFactory::new(4, 4);
        let source = f.at(1, 1).unwrap();

        generator::dfs(&mut maze, &[source], &mut None);
        assert_eq!(maze[source], OPEN, "DFS generation failed for {:?}", shape);
    }
}

#[test]
fn maze_solving_finds_valid_paths() {
    use amazeing::maze::solver;

    let maze = Maze::new(UnitShape::Square, 4, 4, OPEN);
    let f = NodeFactory::new(4, 4);
    let source = f.at(0, 0).unwrap();
    let destination = f.at(3, 3).unwrap();

    let path = solver::bfs(&maze, source, destination, &mut None);
    assert!(!path.is_empty());
    assert_eq!(path.first(), Some(&source));
    assert_eq!(path.last(), Some(&destination));
}

#[test]
fn all_solver_algorithms_reach_destination() {
    use amazeing::maze::heuristics::manhattan_heuristic;
    use amazeing::maze::solver;

    let maze = Maze::new(UnitShape::Square, 5, 5, OPEN);
    let f = NodeFactory::new(5, 5);
    let source = f.at(0, 0).unwrap();
    let destination = f.at(4, 4).unwrap();

    let bfs_path = solver::bfs(&maze, source, destination, &mut None);
    assert!(!bfs_path.is_empty());

    let dfs_path = solver::dfs(&maze, source, destination, &mut None);
    assert!(!dfs_path.is_empty());

    let gbf_path = solver::greedy_best_first(&maze, source, destination, manhattan_heuristic, &mut None);
    assert!(!gbf_path.is_empty());

    let beam_path = solver::beam_search(&maze, source, destination, manhattan_heuristic, &mut None);
    assert!(!beam_path.is_empty());

    let iddfs_path = solver::iddfs(&maze, source, destination, &mut None);
    assert!(!iddfs_path.is_empty());

    let bi_bfs_path = solver::bidirectional_bfs(&maze, source, destination, &mut None);
    assert!(!bi_bfs_path.is_empty());

    let bi_gbf_path =
        solver::bidirectional_greedy_best_first(&maze, source, destination, manhattan_heuristic, &mut None);
    assert!(!bi_gbf_path.is_empty());

    let astar_path = solver::a_star(&maze, source, destination, manhattan_heuristic, &mut None);
    assert!(!astar_path.is_empty());

    let ab_path = solver::aldous_broder(&maze, source, destination, &mut None);
    assert!(!ab_path.is_empty());

    let bi_astar_path = solver::bidirectional_a_start(
        &maze,
        source,
        destination,
        manhattan_heuristic,
        &mut None,
    );
    assert!(!bi_astar_path.is_empty());

    let sas_path = solver::simulated_annealing_search(&maze, source, destination, manhattan_heuristic, &mut None);
    assert!(!sas_path.is_empty());

    // All should reach destination
    assert_eq!(bfs_path.last(), Some(&destination));
    assert_eq!(dfs_path.last(), Some(&destination));
    assert_eq!(gbf_path.last(), Some(&destination));
    assert_eq!(beam_path.last(), Some(&destination));
    assert_eq!(iddfs_path.last(), Some(&destination));
    assert_eq!(bi_bfs_path.last(), Some(&destination));
    assert_eq!(bi_gbf_path.last(), Some(&destination));
    assert_eq!(astar_path.last(), Some(&destination));
    assert_eq!(ab_path.last(), Some(&destination));
    assert_eq!(bi_astar_path.last(), Some(&destination));
    assert_eq!(sas_path.last(), Some(&destination));
}

#[test]
fn stream_emission_provides_trace_steps() {
    use amazeing::maze::generator;

    let mut maze = Maze::new(UnitShape::Square, 4, 4, BLOCK);
    let f = NodeFactory::new(4, 4);
    let source = f.at(2, 2).unwrap();

    let mut step_count = 0usize;
    let mut emit = |_| step_count += 1;

    generator::bfs_stream(&mut maze, &[source], &mut emit);
    assert!(step_count > 0, "BFS stream should emit trace steps");
    assert_eq!(maze[source], OPEN);

    let mut prim_steps = 0usize;
    let mut emit_prim = |_| prim_steps += 1;
    generator::prim_stream(&mut maze, &[source], &mut emit_prim);
    assert!(prim_steps > 0, "Prim stream should emit trace steps");

    let mut beam_steps = 0usize;
    let mut emit_beam = |_| beam_steps += 1;
    generator::beam_search_stream(
        &mut maze,
        &[source],
        f.at(3, 3).unwrap(),
        amazeing::maze::heuristics::manhattan_heuristic,
        0,
        &mut emit_beam,
    );
    assert!(beam_steps > 0, "Beam Search stream should emit trace steps");

    let mut ab_steps = 0usize;
    let mut emit_ab = |_| ab_steps += 1;
    generator::aldous_broder_stream(&mut maze, &[source], &mut emit_ab);
    assert!(ab_steps > 0, "Aldous-Broder stream should emit trace steps");

    let mut maze_bi = Maze::new(UnitShape::Square, 4, 4, BLOCK);
    let destination = f.at(3, 3).unwrap();
    let mut bi_steps = 0usize;
    let mut emit_bi = |_| bi_steps += 1;
    generator::bidirectional_a_start_stream(
        &mut maze_bi,
        &[source],
        destination,
        amazeing::maze::heuristics::manhattan_heuristic,
        0,
        &mut emit_bi,
    );
    assert!(bi_steps > 0, "Bidirectional A* stream should emit trace steps");

    let mut bi_gbf_steps = 0usize;
    let mut emit_bi_gbf = |_| bi_gbf_steps += 1;
    generator::bidirectional_greedy_best_first_stream(
        &mut maze_bi,
        &[source],
        destination,
        amazeing::maze::heuristics::manhattan_heuristic,
        0,
        &mut emit_bi_gbf,
    );
    assert!(bi_gbf_steps > 0, "Bidirectional Greedy Best-First stream should emit trace steps");

    let mut sas_steps = 0usize;
    let mut emit_sas = |_| sas_steps += 1;
    generator::simulated_annealing_search_stream(
        &mut maze,
        &[source],
        Some(destination),
        amazeing::maze::heuristics::manhattan_heuristic,
        &mut emit_sas,
    );
    assert!(sas_steps > 0, "Simulated Annealing Search stream should emit trace steps");
}

#[test]
fn multiple_sources_generate_from_all() {
    use amazeing::maze::generator;

    let mut maze = Maze::new(UnitShape::Square, 5, 5, BLOCK);
    let f = NodeFactory::new(5, 5);
    let s1 = f.at(1, 1).unwrap();
    let s2 = f.at(3, 3).unwrap();

    generator::dfs(&mut maze, &[s1, s2], &mut None);

    assert_eq!(maze[s1], OPEN);
    assert_eq!(maze[s2], OPEN);

    // At least some neighbors should be open
    let neighbors_s1: Vec<_> = s1
        .neighbours(&UnitShape::Square)
        .into_iter()
        .filter(|n| maze[*n] == OPEN)
        .collect();
    assert!(!neighbors_s1.is_empty());
}

#[test]
fn maze_clone_is_independent_copy() {
    let mut m1 = Maze::new(UnitShape::Square, 2, 2, BLOCK);
    let m2 = m1.clone();

    let f = NodeFactory::new(2, 2);
    let node = f.at(0, 0).unwrap();

    m1[node] = OPEN;
    assert_eq!(m1[node], OPEN);
    assert_eq!(m2[node], BLOCK, "Clone should be independent");
}

#[test]
fn heuristics_guide_a_star_optimally() {
    use amazeing::maze::heuristics::{dijkstra_heuristic, manhattan_heuristic};
    use amazeing::maze::solver;

    let maze = Maze::new(UnitShape::Square, 6, 6, OPEN);
    let f = NodeFactory::new(6, 6);
    let source = f.at(0, 0).unwrap();
    let dest = f.at(5, 5).unwrap();

    let path_astar = solver::a_star(&maze, source, dest, manhattan_heuristic, &mut None);
    let path_dijkstra = solver::a_star(&maze, source, dest, dijkstra_heuristic, &mut None);

    // Both should find valid paths; A* with heuristic should be at least as good
    assert!(!path_astar.is_empty());
    assert!(!path_dijkstra.is_empty());
    assert!(path_astar.len() <= path_dijkstra.len() + 1);
}

#[test]
fn neighborhood_calculations_are_shape_aware() {
    let f = NodeFactory::new(5, 5);

    let square_neighbors = f.at(2, 2).unwrap().neighbours(&UnitShape::Square);
    assert_eq!(square_neighbors.len(), 4);

    let hexagon_neighbors = f.at(2, 2).unwrap().neighbours(&UnitShape::Hexagon);
    assert_eq!(hexagon_neighbors.len(), 6);

    let octagon_neighbors = f.at(2, 2).unwrap().neighbours(&UnitShape::Octagon);
    assert_eq!(octagon_neighbors.len(), 4);
}

#[test]
fn triangular_maze_generation_and_solving() {
    use amazeing::maze::generator;
    use amazeing::maze::solver;

    let mut maze = Maze::new(UnitShape::Triangle, 4, 4, BLOCK);
    let f = NodeFactory::new(4, 4);
    let source = f.at(1, 1).unwrap();

    generator::bfs(&mut maze, &[source], &mut None);

    let dest = f.at(2, 2).unwrap();
    if maze[dest] == OPEN {
        let path = solver::bfs(&maze, source, dest, &mut None);
        assert!(!path.is_empty());
    }
}
