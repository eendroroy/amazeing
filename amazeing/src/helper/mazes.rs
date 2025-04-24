use amazeing::tiled::{BLOCK, Maze, MazeShape, UnitShape, VOID};

pub(crate) fn generate_maze_tiles(rows: usize, cols: usize, maze_shape: MazeShape, unit_shape: UnitShape) -> Maze {
    let mut data = vec![vec![BLOCK; cols]; rows];
    match (maze_shape, unit_shape) {
        (MazeShape::Triangle, UnitShape::Square) => set_triangle_square_perimeter(&mut data, rows, cols),
        (MazeShape::Triangle, UnitShape::Hexagon) => set_triangle_hexagon_circle_perimeter(&mut data, rows, cols),
        (MazeShape::Triangle, UnitShape::Circle) => set_triangle_hexagon_circle_perimeter(&mut data, rows, cols),
        _ => {}
    };

    Maze::from(maze_shape, unit_shape, data)
}

fn set_triangle_square_perimeter(data: &mut Vec<Vec<i8>>, rows: usize, cols: usize) {
    let centre_column = cols.div_ceil(2);
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

fn set_triangle_hexagon_circle_perimeter(data: &mut Vec<Vec<i8>>, rows: usize, cols: usize) {
    let centre_column = cols.div_ceil(2);
    for r in 0..rows {
        let r_val = ((r + 1) as f32 / 2.).floor() as usize;
        if r_val < centre_column {
            for c in 1..(centre_column - r_val) {
                data[r][c - 1] = VOID;
            }
            if r % 2 == 0 {
                for c in 1..(centre_column - r_val) {
                    data[r][cols - c] = VOID;
                }
            } else {
                for c in 1..=(centre_column - r_val) {
                    data[r][cols - c] = VOID;
                }
            }
        }
    }
}
