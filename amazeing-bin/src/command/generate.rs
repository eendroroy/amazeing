use crate::context::{GEN_CTX, VIS_CTX};
use crate::helper::dumper::dump_maze_to_file;
use crate::helper::{generate_maze, random_node};
use crate::ui;
use amazeing::maze::matrix::Maze;

pub(crate) fn generate(simulate: bool, visualize: bool) {
    if simulate {
        ui::generate_simulate::main()
    } else {
        let (rows, cols) = (GEN_CTX.read().unwrap().rows, GEN_CTX.read().unwrap().cols);

        let source = random_node((GEN_CTX.read().unwrap().rows, GEN_CTX.read().unwrap().cols));

        println!("Starting at {:?}", source);

        let mut maze = Maze::from(vec![vec![0u32; cols]; rows]);

        generate_maze(
            &mut maze,
            source,
            &GEN_CTX.read().unwrap().procedure,
            &mut None,
        );

        dump_maze_to_file(&GEN_CTX.read().unwrap().maze_file_path, &maze);

        if visualize {
            VIS_CTX.write().unwrap().maze = maze;
            ui::visualize::main()
        }
    }
}
