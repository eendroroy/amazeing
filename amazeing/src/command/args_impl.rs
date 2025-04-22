use crate::command::{ArgHeuristic, ArgUnitShape};
use amazeing::tiled::heuristics::*;
use amazeing::tiled::{NodeHeuFn, UnitShape};

impl ArgUnitShape {
    pub fn as_unit_shape(&self) -> UnitShape {
        match self {
            ArgUnitShape::Triangle => UnitShape::Triangle,
            ArgUnitShape::Square => UnitShape::Square,
            ArgUnitShape::Hexagon => UnitShape::Hexagon,
            ArgUnitShape::Circle => UnitShape::Circle,
        }
    }
}

impl ArgHeuristic {
    pub fn as_node_heu_fn(&self) -> NodeHeuFn {
        match self {
            ArgHeuristic::Manhattan => manhattan_heuristic,
            ArgHeuristic::Euclidean => euclidean_heuristic,
            ArgHeuristic::Chebyshev => chebyshev_heuristic,
            ArgHeuristic::Octile => octile_heuristic,
            ArgHeuristic::Dijkstra => dijkstra_heuristic,
        }
    }
}
