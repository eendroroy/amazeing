use crate::context::{Colors, GeneratorContext};
use crate::generator;
use crate::matrix::dumper::dump_maze_to_file;
use amazeing::generator::matrix::{bfs, dfs};
use amazeing::maze::matrix::Maze;
use rand::random_range;
use std::sync::{LazyLock, RwLock};

pub static COLORS: LazyLock<Colors> = LazyLock::new(|| Colors::new());
pub static GENERATOR_CONTEXT: LazyLock<RwLock<GeneratorContext>> =
    LazyLock::new(|| RwLock::new(GeneratorContext::new()));

pub(crate) fn generate(maze_file_path: String, rows: String, cols: String, proc: String) {
    GENERATOR_CONTEXT.write().unwrap().maze_file_path = maze_file_path.clone();

    if rows != String::new() {
        GENERATOR_CONTEXT.write().unwrap().rows = usize::from_str_radix(rows.as_str(), 10).unwrap();
    }

    if cols != String::new() {
        GENERATOR_CONTEXT.write().unwrap().cols = usize::from_str_radix(cols.as_str(), 10).unwrap();
    }

    match proc.as_str() {
        "manual" => generator::manual::main(),
        name => {
            let (rows, cols) = (
                GENERATOR_CONTEXT.read().unwrap().rows,
                GENERATOR_CONTEXT.read().unwrap().cols,
            );

            let start = (random_range(0..rows), random_range(0..cols));

            println!("Starting at {:?}", start);

            let mut maze = Maze::from(vec![vec![0u32; cols]; rows]);
            match name {
                "bfs" => bfs(&mut maze, start, &mut None),
                "dfs" => dfs(&mut maze, start, &mut None),
                _ => panic!("Unknown procedure: {}", proc),
            };
            dump_maze_to_file(&*maze_file_path.clone(), maze);
            generator::manual::main()
        }
    }
}
