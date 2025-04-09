use super::Node;

/// Manhattan distance heuristic for A*
///
/// This heuristic calculates the Manhattan distance between two nodes.
/// It is the sum of the absolute differences of their Cartesian coordinates.
///
/// # Arguments
///
/// * `node` - The starting node.
/// * `goal` - The goal node.
///
/// # Returns
///
/// The Manhattan distance as a `u32`.
pub fn manhattan_heuristic(node: Node, goal: Node) -> u32 {
    let ((x1, y1), (x2, y2)) = get_points(node, goal);
    ((x1 - x2).abs() + (y1 - y2).abs()) as u32
}

/// Euclidean distance heuristic for A*
///
/// This heuristic calculates the Euclidean distance between two nodes.
/// It is the straight-line distance between the points in Euclidean space.
///
/// # Arguments
///
/// * `node` - The starting node.
/// * `goal` - The goal node.
///
/// # Returns
///
/// The Euclidean distance as a `u32`.
pub fn euclidean_heuristic(node: Node, goal: Node) -> u32 {
    let ((x1, y1), (x2, y2)) = get_points(node, goal);
    ((x1 as f64 - x2 as f64).powi(2) + (y1 as f64 - y2 as f64).powi(2)).sqrt() as u32
}

/// Chebyshev distance heuristic for A*
///
/// This heuristic calculates the Chebyshev distance between two nodes.
/// It is the maximum of the absolute differences of their Cartesian coordinates.
///
/// # Arguments
///
/// * `node` - The starting node.
/// * `goal` - The goal node.
///
/// # Returns
///
/// The Chebyshev distance as a `u32`.
pub fn chebyshev_heuristic(node: Node, goal: Node) -> u32 {
    let ((x1, y1), (x2, y2)) = get_points(node, goal);
    (x1 - x2).abs().max((y1 - y2).abs()) as u32
}

/// Octile distance heuristic for A*
///
/// This heuristic calculates the Octile distance between two nodes.
/// It is a combination of the Manhattan and Chebyshev distances, often used in grid-based pathfinding.
///
/// # Arguments
///
/// * `node` - The starting node.
/// * `goal` - The goal node.
///
/// # Returns
///
/// The Octile distance as a `u32`.
pub fn octile_heuristic(node: Node, goal: Node) -> u32 {
    let ((x1, y1), (x2, y2)) = get_points(node, goal);
    let (dx, dy) = ((x1 - x2).abs(), (y1 - y2).abs());
    ((dx + dy) + ((2 - 2_i32.isqrt()) * dx.min(dy))) as u32
}

/// Dijkstra's Heuristic
///
/// This heuristic is used for Dijkstra's algorithm, which does not use a heuristic.
/// It always returns 0.
///
/// # Arguments
///
/// * `node` - The starting node (unused).
/// * `goal` - The goal node (unused).
///
/// # Returns
///
/// Always returns 0.
pub fn dijkstra_heuristic(_: Node, _: Node) -> u32 {
    0
}

fn get_points(node: Node, goal: Node) -> ((i32, i32), (i32, i32)) {
    ((node.0 as i32, node.1 as i32), (goal.0 as i32, goal.1 as i32))
}
