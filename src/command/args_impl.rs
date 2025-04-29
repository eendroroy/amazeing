use crate::_lib::tiled::heuristics::*;
use crate::_lib::tiled::node::WeightDirection;
use crate::_lib::tiled::{NodeHeuFn, UnitShape};
use crate::command::{ArgHeuristic, ArgUnitShape, ArgWeightDirection};

impl ArgUnitShape {
    pub fn as_unit_shape(&self) -> UnitShape {
        match self {
            ArgUnitShape::Triangle => UnitShape::Triangle,
            ArgUnitShape::Square => UnitShape::Square,
            ArgUnitShape::Hexagon => UnitShape::Hexagon,
            ArgUnitShape::Octagon => UnitShape::Octagon,
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

impl ArgWeightDirection {
    pub fn as_weight_direction(&self) -> WeightDirection {
        match self {
            ArgWeightDirection::Forward => WeightDirection::Forward,
            ArgWeightDirection::Backward => WeightDirection::Backward,
        }
    }
}
