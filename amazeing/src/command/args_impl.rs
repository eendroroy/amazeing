use crate::command::{ArgHeuristic, ArgMazeShape, ArgUnitShape};
use amazeing::tiled::heuristics::*;
use amazeing::tiled::{MazeShape, NodeHeuFn, UnitShape};

impl ArgMazeShape {
    pub fn shape(&self) -> MazeShape {
        match self {
            ArgMazeShape::Rectangle => MazeShape::Rectangle,
        }
    }
}

impl ArgUnitShape {
    pub fn shape(&self) -> UnitShape {
        match self {
            ArgUnitShape::Triangle => UnitShape::Triangle,
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
