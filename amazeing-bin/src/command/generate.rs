use crate::cli::ArgGenProcedure;
use crate::context::{GEN_CTX, VIS_CTX};
use crate::helper::dumper::dump_maze_to_file;
use crate::ui;
use amazeing::generator::matrix::{dfs, random};
use amazeing::maze::matrix::Maze;
use rand::random_range;

pub(crate) fn generate(visualize: bool) {
    let (rows, cols) = (GEN_CTX.read().unwrap().rows, GEN_CTX.read().unwrap().cols);

    let start = (random_range(0..rows), random_range(0..cols));

    println!("Starting at {:?}", start);

    let mut maze = Maze::from(vec![vec![0u32; cols]; rows]);
    match GEN_CTX.read().unwrap().procedure {
        ArgGenProcedure::Random => random(&mut maze, start, &mut None),
        ArgGenProcedure::Dfs => dfs(&mut maze, start, &mut None),
    };
    dump_maze_to_file(&GEN_CTX.read().unwrap().maze_file_path, &maze);
    if visualize {
        VIS_CTX.write().unwrap().maze = maze;
        ui::visualize::main()
    }
}
