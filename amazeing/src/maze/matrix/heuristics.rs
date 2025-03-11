use crate::DNode;

/// Manhattan distance heuristic for A*
pub fn manhattan_heuristic(node: DNode, goal: DNode) -> u32 {
    let ((x1, y1), (x2, y2)) = get_points(node, goal);
    ((x1 - x2).abs() + (y1 - y2).abs()) as u32
}

/// Euclidean distance heuristic for A*
pub fn euclidean_heuristic(node: DNode, goal: DNode) -> u32 {
    let ((x1, y1), (x2, y2)) = get_points(node, goal);
    ((x1 as f64 - x2 as f64).powi(2) + (y1 as f64 - y2 as f64).powi(2)).sqrt() as u32
}

/// Chebyshev distance heuristic for A*
pub fn chebyshev_heuristic(node: DNode, goal: DNode) -> u32 {
    let ((x1, y1), (x2, y2)) = get_points(node, goal);
    (x1 - x2).abs().max((y1 - y2).abs()) as u32
}

/// Octile distance heuristic for A*
pub fn octile_heuristic(node: DNode, goal: DNode) -> u32 {
    let ((x1, y1), (x2, y2)) = get_points(node, goal);
    let (dx, dy) = ((x1 - x2).abs(), (y1 - y2).abs());
    ((dx + dy) + ((2 - 2_i32.isqrt()) * dx.min(dy))) as u32
}

/// Dijkstra's Heuristic
pub fn dijkstra_heuristic(_: DNode, _: DNode) -> u32 {
    0
}

fn get_points(node: DNode, goal: DNode) -> ((i32, i32), (i32, i32)) {
    (
        (node.0 as i32, node.1 as i32),
        (goal.0 as i32, goal.1 as i32),
    )
}
