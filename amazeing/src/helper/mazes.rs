use amazeing::tiled::{BLOCK, Maze, MazeData, MazeShape, UnitShape, VOID};

pub(crate) fn generate_maze_tiles(rows: usize, cols: usize, maze_shape: MazeShape, unit_shape: UnitShape) -> Maze {
    let mut data: MazeData;
    if maze_shape == MazeShape::Triangle {
        data = vec![vec![VOID; cols]; rows];
        match unit_shape {
            UnitShape::Triangle => set_triangle_triangle_perimeter(&mut data),
            UnitShape::Square => set_triangle_square_perimeter(&mut data),
            UnitShape::Hexagon => set_triangle_hexagon_circle_perimeter(&mut data),
            UnitShape::Circle => set_triangle_hexagon_circle_perimeter(&mut data),
        }
    } else {
        data = vec![vec![VOID; cols]; rows]
    }

    Maze::from(maze_shape, unit_shape, data)
}

fn set_triangle_square_perimeter(data: &mut MazeData) {
    let cols: usize = data.first().unwrap().len();
    let centre_column = cols.div_ceil(2) - 1;
    for (r, row) in data.iter_mut().enumerate() {
        let r_val = ((r + 1) as f32 / 2.).floor() as usize;
        ((centre_column - r_val)..=(centre_column + r_val)).for_each(|c| row[c] = BLOCK);
    }
}

fn set_triangle_hexagon_circle_perimeter(data: &mut MazeData) {
    let cols: usize = data.first().unwrap().len();
    let centre_column = cols.div_ceil(2);
    for (r, row) in data.iter_mut().enumerate() {
        let r_val = ((r + 1) as f32 / 2.).floor() as usize;
        let range = if r % 2 == 0 {
            (centre_column - r_val - 1)..(centre_column + r_val)
        } else {
            (centre_column - r_val - 1)..(centre_column + r_val - 1)
        };
        range.for_each(|c| row[c] = BLOCK);
    }
}

fn set_triangle_triangle_perimeter(data: &mut MazeData) {
    let cols: usize = data.first().unwrap().len();
    let centre_column = cols.div_ceil(2) - 1;
    for (r, row) in data.iter_mut().enumerate() {
        let r_val = ((r + 1) as f32 / 2.).floor() as usize;
        let range = if r_val % 2 == 0 {
            (centre_column - r_val / 2 + 1)..(centre_column + r_val.div_ceil(2) + 1)
        } else {
            (centre_column - r_val / 2)..(centre_column + r_val.div_ceil(2))
        };
        range.for_each(|c| row[c] = BLOCK);
    }
}
