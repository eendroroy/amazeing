use crate::command::{ArgHeuristic, ArgMazeShape, ArgUnitShape, ArgWeightDirection};
use crate::core::tiled::heuristics::*;
use crate::core::tiled::node::WeightDirection;
use crate::core::tiled::{MazeShape, NodeHeuFn, UnitShape};

impl ArgMazeShape {
    pub fn shape(&self) -> MazeShape {
        match self {
            ArgMazeShape::Triangle(_) => MazeShape::Triangle,
            ArgMazeShape::Rectangle(_) => MazeShape::Rectangle,
            ArgMazeShape::Hexagon(_) => MazeShape::Hexagon,
            ArgMazeShape::Circle(_) => MazeShape::Circle,
        }
    }

    pub fn dimension(&self) -> (usize, usize) {
        match self {
            ArgMazeShape::Triangle(args) => (args.base, args.base),
            ArgMazeShape::Rectangle(args) => (args.rows, args.cols),
            ArgMazeShape::Hexagon(args) => (args.diameter, args.diameter),
            ArgMazeShape::Circle(args) => (args.diameter, args.diameter),
        }
    }
}

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
