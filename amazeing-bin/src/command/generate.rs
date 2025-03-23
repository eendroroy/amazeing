use crate::context::{GEN_CTX, VIS_CTX};
use crate::helper::{dump_maze_to_file, generate_maze};
use crate::ui;
use amazeing::matrix::Maze;

pub(crate) fn generate(simulate: bool, view: bool) {
    if simulate {
        ui::generate_simulate::main()
    } else {
        let (rows, cols) = (GEN_CTX.read().unwrap().rows, GEN_CTX.read().unwrap().cols);

        let mut maze = Maze::from(vec![vec![0u32; cols]; rows]);

        generate_maze(
            &mut maze,
            GEN_CTX.read().unwrap().source,
            &GEN_CTX.read().unwrap().procedure,
            &mut None,
        );

        if let Some(maze_file_path) = GEN_CTX.read().unwrap().maze_file_path.clone() {
            dump_maze_to_file(&maze_file_path, &maze);
        }

        if view {
            VIS_CTX.write().unwrap().maze = maze;
            ui::visualize::main()
        }
    }
}
