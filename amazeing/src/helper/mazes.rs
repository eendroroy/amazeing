use amazeing::tiled::{Maze, MazeShape, UnitShape, BLOCK, VOID};

pub(crate) fn generate_maze_tiles(rows: usize, cols: usize, maze_shape: MazeShape, unit_shape: UnitShape) -> Maze {
    let mut data = vec![vec![BLOCK; cols]; rows];
    match (maze_shape, unit_shape) {
        (MazeShape::Triangle, UnitShape::Square) => {
            let centre_column = (cols + 1) / 2;
            for r in 0..rows {
                let r_val = ((r + 1) as f32 / 2.).floor() as usize;
                if r_val < centre_column {
                    for c in 1..(centre_column - r_val) {
                        data[r][c - 1] = VOID;
                        data[r][cols - c] = VOID;
                    }
                }
            }
        }
        _ => {}
    };

    Maze::from(maze_shape, unit_shape, data)
}
