use crate::command::Proc;
use crate::context::CONTEXT;
use crate::helper::dumper::dump_maze_to_file;
use crate::mode;
use amazeing::generator::matrix::{bfs, dfs};
use amazeing::maze::matrix::Maze;
use rand::random_range;

pub(crate) fn generate() {
    let (rows, cols) = (CONTEXT.read().unwrap().rows, CONTEXT.read().unwrap().cols);

    let start = (random_range(0..rows), random_range(0..cols));

    println!("Starting at {:?}", start);

    let mut maze = Maze::from(vec![vec![0u32; cols]; rows]);
    match CONTEXT.read().unwrap().proc {
        Proc::Bfs => bfs(&mut maze, start, &mut None),
        Proc::Dfs => dfs(&mut maze, start, &mut None),
        _ => panic!("No algorithm provided"),
    };
    dump_maze_to_file(&*CONTEXT.read().unwrap().maze_file_path, &maze);
    CONTEXT.write().unwrap().maze = maze;
    mode::visualize::visualize()
}
