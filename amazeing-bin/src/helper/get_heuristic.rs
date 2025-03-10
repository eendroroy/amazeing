use amazeing::maze::matrix::{
    chebyshev_heuristic, dijkstra_heuristic, euclidean_heuristic, manhattan_heuristic,
    octile_heuristic,
};
use amazeing::DNode;

pub(crate) fn get_heu(heu: &str) -> fn(DNode, DNode) -> u32 {
    match heu {
        "manhattan" => manhattan_heuristic,
        "euclidean" => euclidean_heuristic,
        "chebyshev" => chebyshev_heuristic,
        "octile" => octile_heuristic,
        "dijkstra" => dijkstra_heuristic,
        _ => dijkstra_heuristic,
    }
}
