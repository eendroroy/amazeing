use crate::command::{ArgDisplayDensity, ArgDisplaySize, ArgHeuristic, ArgBlockShape};
use amazeing::matrix::heuristics::{
    chebyshev_heuristic, dijkstra_heuristic, euclidean_heuristic, manhattan_heuristic, octile_heuristic,
};
use amazeing::matrix::{NodeHeuFn, UnitShape};

impl ArgDisplaySize {
    pub fn size(&self, density: f32, shape: f32) -> (f32, f32, f32) {
        match self {
            ArgDisplaySize::Xxs => (3., density * 1., shape * 3.),
            ArgDisplaySize::Xs => (5., density * 1., shape * 5.),
            ArgDisplaySize::S => (10., density * 2., shape * 10.),
            ArgDisplaySize::M => (15., density * 3., shape * 15.),
            ArgDisplaySize::L => (25., density * 4., shape * 20.),
            ArgDisplaySize::Xl => (30., density * 5., shape * 30.),
            ArgDisplaySize::Xxl => (40., density * 6., shape * 40.),
        }
    }
}

impl ArgBlockShape {
    pub fn shape(&self) -> UnitShape {
        match self {
            ArgBlockShape::Square => UnitShape::Square,
            ArgBlockShape::Hexagon => UnitShape::Hexagon,
        }
    }
}

impl ArgDisplayDensity {
    pub fn multiplier(&self) -> f32 {
        match self {
            ArgDisplayDensity::Connected => 0.,
            ArgDisplayDensity::Dense => 1. / 1.5,
            ArgDisplayDensity::Standard => 1.,
            ArgDisplayDensity::Cozy => 1.5,
            ArgDisplayDensity::Ample => 2.,
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
