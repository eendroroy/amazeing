use crate::command::{ArgHeuristic, ArgUnitShape};
use amazeing::matrix::heuristics::{
    chebyshev_heuristic, dijkstra_heuristic, euclidean_heuristic, manhattan_heuristic, octile_heuristic,
};
use amazeing::matrix::{NodeHeuFn, UnitShape};

impl ArgUnitShape {
    pub fn shape(&self) -> UnitShape {
        match self {
            ArgUnitShape::Square => UnitShape::Square,
            ArgUnitShape::Hexagon => UnitShape::Hexagon,
            ArgUnitShape::Circle => UnitShape::Circle,
        }
    }
}

impl ArgHeuristic {
    pub fn heuristic(&self) -> NodeHeuFn {
        match self {
            ArgHeuristic::Manhattan => manhattan_heuristic,
            ArgHeuristic::Euclidean => euclidean_heuristic,
            ArgHeuristic::Chebyshev => chebyshev_heuristic,
            ArgHeuristic::Octile => octile_heuristic,
            ArgHeuristic::Dijkstra => dijkstra_heuristic,
        }
    }
}
