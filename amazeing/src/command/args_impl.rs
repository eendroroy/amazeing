use crate::command::{ArgDisplayDensity, ArgDisplaySize, ArgHeuristic, ArgShape};
use amazeing::matrix::heuristics::{
    chebyshev_heuristic, dijkstra_heuristic, euclidean_heuristic, manhattan_heuristic, octile_heuristic,
};
use amazeing::matrix::{NodeHeuFn, Shape};

impl ArgDisplaySize {
    pub fn size(&self, density: f32) -> (f32, f32, f32) {
        match self {
            ArgDisplaySize::Xxs => (3., density * 1., 3.),
            ArgDisplaySize::Xs => (5., density * 1., 5.),
            ArgDisplaySize::S => (10., density * 2., 10.),
            ArgDisplaySize::M => (15., density * 3., 15.),
            ArgDisplaySize::L => (25., density * 4., 20.),
            ArgDisplaySize::Xl => (30., density * 5., 30.),
            ArgDisplaySize::Xxl => (40., density * 6., 40.),
        }
    }
}

impl ArgShape {
    pub fn shape(&self) -> Shape {
        match self {
            ArgShape::Square => Shape::Square,
            ArgShape::Hexagon => Shape::Hexagon,
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
