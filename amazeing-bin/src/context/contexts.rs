use crate::context::{Colors, GeneratorContext, SolverContext};
use std::sync::{LazyLock, RwLock};

type CtxColor = LazyLock<Colors>;
type CtxSolve = LazyLock<RwLock<SolverContext>>;
type CtxGen = LazyLock<RwLock<GeneratorContext>>;

pub static COLORS: CtxColor = LazyLock::new(|| Colors::new());
pub static SOLVER_CONTEXT: CtxSolve = LazyLock::new(|| RwLock::new(SolverContext::new()));
pub static GENERATOR_CONTEXT: CtxGen = LazyLock::new(|| RwLock::new(GeneratorContext::new()));
