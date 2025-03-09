use crate::context::{Colors, GeneratorContext, SolverContext};
use std::sync::{LazyLock, RwLock};

pub static COLORS: LazyLock<Colors> = LazyLock::new(|| Colors::new());
pub static SOLVER_CONTEXT: LazyLock<RwLock<SolverContext>> =
    LazyLock::new(|| RwLock::new(SolverContext::new()));
pub static GENERATOR_CONTEXT: LazyLock<RwLock<GeneratorContext>> =
    LazyLock::new(|| RwLock::new(GeneratorContext::new()));
