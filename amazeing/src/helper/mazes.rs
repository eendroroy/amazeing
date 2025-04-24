use amazeing::tiled::{BLOCK, Maze, MazeShape, UnitShape, VOID};

pub(crate) fn generate_maze_tiles(rows: usize, cols: usize, maze_shape: MazeShape, unit_shape: UnitShape) -> Maze {
    let mut data = vec![vec![BLOCK; cols]; rows];
    if maze_shape == MazeShape::Triangle {
        match unit_shape {
            UnitShape::Square => set_triangle_square_perimeter(&mut data),
            UnitShape::Hexagon => set_triangle_hexagon_circle_perimeter(&mut data),
            UnitShape::Circle => set_triangle_hexagon_circle_perimeter(&mut data),
            _ => {}
        }
    };

    Maze::from(maze_shape, unit_shape, data)
}

fn set_triangle_square_perimeter(data: &mut [Vec<i8>]) {
    let cols: usize = data.first().unwrap().len();
    let centre_column = cols.div_ceil(2);
    for (r, row) in data.iter_mut().enumerate() {
        let r_val = ((r + 1) as f32 / 2.).floor() as usize;
        if r_val < centre_column {
            for c in 1..(centre_column - r_val) {
                row[c - 1] = VOID;
                row[cols - c] = VOID;
            }
        }
    }
}

fn set_triangle_hexagon_circle_perimeter(data: &mut [Vec<i8>]) {
    let cols: usize = data.first().unwrap().len();
    let centre_column = cols.div_ceil(2);
    for (r, row) in data.iter_mut().enumerate() {
        let r_val = ((r + 1) as f32 / 2.).floor() as usize;
        if r_val < centre_column {
            for c in 1..(centre_column - r_val) {
                row[c - 1] = VOID;
            }
            if r % 2 == 0 {
                for c in 1..(centre_column - r_val) {
                    row[cols - c] = VOID;
                }
            } else {
                for c in 1..=(centre_column - r_val) {
                    row[cols - c] = VOID;
                }
            }
        }
    }
}
