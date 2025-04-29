use crate::core::tiled::node::Node;

pub fn manhattan_heuristic(node: Node, goal: Node) -> u32 {
    let ((x1, y1), (x2, y2)) = get_points(node, goal);
    ((x1 - x2).abs() + (y1 - y2).abs()) as u32
}

pub fn euclidean_heuristic(node: Node, goal: Node) -> u32 {
    let ((x1, y1), (x2, y2)) = get_points(node, goal);
    ((x1 as f64 - x2 as f64).powi(2) + (y1 as f64 - y2 as f64).powi(2)).sqrt() as u32
}

pub fn chebyshev_heuristic(node: Node, goal: Node) -> u32 {
    let ((x1, y1), (x2, y2)) = get_points(node, goal);
    (x1 - x2).abs().max((y1 - y2).abs()) as u32
}

pub fn octile_heuristic(node: Node, goal: Node) -> u32 {
    let ((x1, y1), (x2, y2)) = get_points(node, goal);
    let (dx, dy) = ((x1 - x2).abs(), (y1 - y2).abs());
    ((dx + dy) + ((2 - 2_i32.isqrt()) * dx.min(dy))) as u32
}

pub fn dijkstra_heuristic(_: Node, _: Node) -> u32 {
    0
}

fn get_points(node: Node, goal: Node) -> ((i32, i32), (i32, i32)) {
    ((node.row as i32, node.col as i32), (goal.row as i32, goal.col as i32))
}
