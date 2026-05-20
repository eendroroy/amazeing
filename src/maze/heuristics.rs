use crate::maze::node::Node;

// ── precision scale ───────────────────────────────────────────────────────────

/// All non-zero heuristics are multiplied by this constant before being
/// returned as `u32`.  Without scaling, two nodes at distances 5.1 and 5.9
/// both round to 5 and are treated identically by the priority queue.
/// Scaling by 100 preserves two decimal places of sub-cell precision, so the
/// algorithm can distinguish "slightly closer" from "slightly farther" nodes
/// and produces more visually directed traversal.
///
/// **Note for generators**: the `jumble_factor` CLI argument adds noise on the
/// scale `0..=jumble_factor`.  With SCALE = 100, meaningful visual randomness
/// requires a jumble_factor in the range 50–500 rather than the old 1–10.
const SCALE: u32 = 100;

// ── heuristic functions ───────────────────────────────────────────────────────

/// Sum of absolute row and column differences (× SCALE).
/// Admissible for 4-directional square/rhombus grids.
pub fn manhattan_heuristic(node: Node, goal: Node) -> u32 {
    let (dr, dc) = delta(node, goal);
    (dr + dc) * SCALE
}

/// Euclidean straight-line distance (× SCALE).
/// Always admissible; less directed than Manhattan for grid movement.
pub fn euclidean_heuristic(node: Node, goal: Node) -> u32 {
    let (dr, dc) = delta(node, goal);
    ((dr as f64).hypot(dc as f64) * SCALE as f64) as u32
}

/// Maximum of the two absolute differences (× SCALE).
/// Admissible for 8-directional grids with unit-cost diagonal movement.
pub fn chebyshev_heuristic(node: Node, goal: Node) -> u32 {
    let (dr, dc) = delta(node, goal);
    dr.max(dc) * SCALE
}

/// Standard octile distance: `max + (√2 − 1)·min` (× SCALE).
/// Admissible for 8-directional grids where diagonal movement costs √2.
pub fn octile_heuristic(node: Node, goal: Node) -> u32 {
    let (dr, dc) = delta(node, goal);
    let (major, minor) = if dr >= dc { (dr, dc) } else { (dc, dr) };
    ((major as f64 + (std::f64::consts::SQRT_2 - 1.0) * minor as f64) * SCALE as f64) as u32
}

/// Cube-coordinate distance for hexagonal (offset) grids.
///
/// The project uses an **odd-r** hex layout (odd rows shifted right).
/// Converting to cube coordinates and taking the Manhattan distance in
/// cube space gives the exact minimum number of hex steps, yielding a
/// perfectly admissible and visually accurate heuristic for hex mazes.
pub fn hex_heuristic(node: Node, goal: Node) -> u32 {
    let (q1, r1, s1) = offset_to_cube(node.row as i32, node.col as i32);
    let (q2, r2, s2) = offset_to_cube(goal.row as i32, goal.col as i32);
    // Cube distance = (|Δq| + |Δr| + |Δs|) / 2
    let dist = ((q1 - q2).unsigned_abs() + (r1 - r2).unsigned_abs() + (s1 - s2).unsigned_abs()) / 2;
    dist * SCALE
}

/// Always returns 0 — degrades A* to Dijkstra / uniform-cost search.
pub fn dijkstra_heuristic(_: Node, _: Node) -> u32 {
    0
}

// ── internal helpers ──────────────────────────────────────────────────────────

/// Returns `(|Δrow|, |Δcol|)` as unsigned integers.
#[inline]
fn delta(a: Node, b: Node) -> (u32, u32) {
    let dr = (a.row as i32 - b.row as i32).unsigned_abs();
    let dc = (a.col as i32 - b.col as i32).unsigned_abs();
    (dr, dc)
}

/// Convert odd-r offset hex coordinates to cube coordinates.
///
/// In odd-r layout, odd rows are shifted one half-cell to the right.
/// Cube coordinates satisfy `q + r + s = 0` and give an exact distance metric.
#[inline]
fn offset_to_cube(row: i32, col: i32) -> (i32, i32, i32) {
    let q = col - (row - (row & 1)) / 2;
    let r = row;
    let s = -q - r;
    (q, r, s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::maze::NodeFactory;

    fn nodes() -> (Node, Node) {
        let f = NodeFactory::new(10, 10);
        (f.at(1, 2).unwrap(), f.at(4, 6).unwrap())
    }

    #[test]
    fn heuristics_return_expected_values() {
        let (a, b) = nodes();
        // dr = |1-4| = 3,  dc = |2-6| = 4  →  hypotenuse = 5.0

        // Manhattan: (3+4) * 100 = 700
        assert_eq!(manhattan_heuristic(a, b), 700);

        // Euclidean: sqrt(9+16) * 100 = 5.0 * 100 = 500
        assert_eq!(euclidean_heuristic(a, b), 500);

        // Chebyshev: max(3,4) * 100 = 400
        assert_eq!(chebyshev_heuristic(a, b), 400);

        // Octile: (4 + (√2-1)*3) * 100 = (4 + 1.2426…) * 100 = 524
        assert_eq!(octile_heuristic(a, b), 524);

        // Dijkstra: always 0
        assert_eq!(dijkstra_heuristic(a, b), 0);
    }

    #[test]
    fn hex_heuristic_exact_for_adjacent_cells() {
        let f = NodeFactory::new(10, 10);
        let center = f.at(4, 4).unwrap();

        // Every direct (step=1) hex neighbor must be exactly SCALE away.
        for neighbor in [
            f.at(4, 5), // right
            f.at(5, 4), // down
            f.at(5, 3), // left-down (even row)
            f.at(4, 3), // left
            f.at(3, 3), // left-up  (even row)
            f.at(3, 4), // up
        ]
        .into_iter()
        .flatten()
        {
            assert_eq!(
                hex_heuristic(center, neighbor),
                SCALE,
                "center→{:?} should be exactly SCALE={}",
                neighbor,
                SCALE
            );
        }
    }

    #[test]
    fn all_heuristics_are_non_negative_and_zero_for_same_node() {
        let f = NodeFactory::new(5, 5);
        let n = f.at(2, 3).unwrap();
        assert_eq!(manhattan_heuristic(n, n), 0);
        assert_eq!(euclidean_heuristic(n, n), 0);
        assert_eq!(chebyshev_heuristic(n, n), 0);
        assert_eq!(octile_heuristic(n, n), 0);
        assert_eq!(hex_heuristic(n, n), 0);
        assert_eq!(dijkstra_heuristic(n, n), 0);
    }

    #[test]
    fn heuristic_ordering_matches_expected_dominance() {
        // For straight-line movement: chebyshev ≤ euclidean ≤ octile ≤ manhattan
        let (a, b) = nodes();
        let cheb = chebyshev_heuristic(a, b);
        let eucl = euclidean_heuristic(a, b);
        let oct  = octile_heuristic(a, b);
        let manh = manhattan_heuristic(a, b);
        assert!(cheb <= eucl, "chebyshev ({cheb}) should be ≤ euclidean ({eucl})");
        assert!(eucl <= oct,  "euclidean ({eucl}) should be ≤ octile ({oct})");
        assert!(oct  <= manh, "octile ({oct}) should be ≤ manhattan ({manh})");
    }
}
