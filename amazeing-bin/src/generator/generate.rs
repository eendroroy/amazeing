use crate::context::{Colors, GeneratorContext};
use crate::generator;
use std::sync::{LazyLock, RwLock};

pub static COLORS: LazyLock<Colors> = LazyLock::new(|| Colors::new());
pub static GENERATOR_CONTEXT: LazyLock<RwLock<GeneratorContext>> =
    LazyLock::new(|| RwLock::new(GeneratorContext::new()));

pub(crate) fn generate(maze_file_path: String, rows: String, cols: String) {
    GENERATOR_CONTEXT.write().unwrap().maze_file_path = maze_file_path.clone();

    if rows != String::new() {
        GENERATOR_CONTEXT.write().unwrap().rows = usize::from_str_radix(rows.as_str(), 10).unwrap();
    }

    if cols != String::new() {
        GENERATOR_CONTEXT.write().unwrap().cols = usize::from_str_radix(cols.as_str(), 10).unwrap();
    }

    generator::manual::main()
}
