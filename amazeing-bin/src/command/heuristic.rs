use amazeing::maze::matrix::{
    chebyshev_heuristic, dijkstra_heuristic, euclidean_heuristic, manhattan_heuristic,
    octile_heuristic,
};
use amazeing::Heu;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Heuristic {
    Manhattan,
    Euclidean,
    Chebyshev,
    Octile,
    Dijkstra,
}

pub fn get_heuristic(str: &str) -> Heuristic {
    match str {
        "manhattan" => Heuristic::Manhattan,
        "euclidean" => Heuristic::Euclidean,
        "chebyshev" => Heuristic::Chebyshev,
        "octile" => Heuristic::Octile,
        "dijkstra" => Heuristic::Dijkstra,
        _ => Heuristic::Dijkstra,
    }
}

pub fn get_heuristic_fn(heuristic: Heuristic) -> Heu {
    match heuristic {
        Heuristic::Manhattan => manhattan_heuristic,
        Heuristic::Euclidean => euclidean_heuristic,
        Heuristic::Chebyshev => chebyshev_heuristic,
        Heuristic::Octile => octile_heuristic,
        Heuristic::Dijkstra => dijkstra_heuristic,
    }
}
