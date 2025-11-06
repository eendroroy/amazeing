use crate::command::{ArgHeuristic, ArgUnitShape, ArgWeightDirection};
use crate::core::tiled::heuristics::*;
use crate::core::tiled::node::WeightDirection;
use crate::core::tiled::{NodeHeuFn, UnitShape};

impl ArgUnitShape {
    pub fn shape(&self) -> UnitShape {
        match self {
            ArgUnitShape::Triangle => UnitShape::Triangle,
            ArgUnitShape::Square => UnitShape::Square,
            ArgUnitShape::Hexagon => UnitShape::Hexagon,
            ArgUnitShape::HexagonRectangle => UnitShape::HexagonRectangle,
            ArgUnitShape::Octagon => UnitShape::Octagon,
            ArgUnitShape::OctagonSquare => UnitShape::OctagonSquare,
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

impl ArgWeightDirection {
    pub fn direction(&self) -> WeightDirection {
        match self {
            ArgWeightDirection::Forward => WeightDirection::Forward,
            ArgWeightDirection::Backward => WeightDirection::Backward,
        }
    }
}
