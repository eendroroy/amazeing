use crate::command::{ArgDisplaySize, ArgHeuristic, ArgUnitShape};
use amazeing::matrix::heuristics::{
    chebyshev_heuristic, dijkstra_heuristic, euclidean_heuristic, manhattan_heuristic, octile_heuristic,
};
use amazeing::matrix::{NodeHeuFn, UnitShape};

impl ArgDisplaySize {
    pub fn size(&self, unit_shape: f32) -> (f32, f32, f32) {
        match self {
            ArgDisplaySize::Xxs => (3., 1., unit_shape * 3.),
            ArgDisplaySize::Xs => (5., 1., unit_shape * 5.),
            ArgDisplaySize::S => (10., 2., unit_shape * 10.),
            ArgDisplaySize::M => (15., 3., unit_shape * 15.),
            ArgDisplaySize::L => (25., 4., unit_shape * 20.),
            ArgDisplaySize::Xl => (30., 5., unit_shape * 30.),
            ArgDisplaySize::Xxl => (40., 6., unit_shape * 40.),
        }
    }
}

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
