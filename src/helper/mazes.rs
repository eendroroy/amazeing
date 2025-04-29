use crate::_lib::tiled::{BLOCK, Maze, MazeData, MazeShape, Node, UnitShape, VOID};
use crate::context::DrawContext;
use macroquad::math::Vec2;

pub(crate) fn generate_maze_tiles(rows: usize, cols: usize, draw_ctx: &DrawContext) -> Maze {
    let mut data: MazeData;
    if draw_ctx.maze_shape == MazeShape::Triangle {
        data = vec![vec![VOID; cols]; rows];
        match draw_ctx.unit_shape {
            UnitShape::Triangle => set_triangle_triangle_perimeter(&mut data),
            UnitShape::Square | UnitShape::Octagon => set_triangle_square_perimeter(&mut data),
            UnitShape::Hexagon => set_triangle_hexagon_perimeter(&mut data),
        }
    } else if draw_ctx.maze_shape == MazeShape::Circle {
        data = vec![vec![VOID; cols]; rows];
        match draw_ctx.unit_shape {
            UnitShape::Triangle => set_circle_triangle_perimeter(&mut data, draw_ctx),
            UnitShape::Square | UnitShape::Octagon => set_circle_square_perimeter(&mut data),
            UnitShape::Hexagon => set_circle_hexagon_perimeter(&mut data, draw_ctx),
        }
    } else {
        data = vec![vec![BLOCK; cols]; rows]
    }

    Maze::from(draw_ctx.maze_shape, draw_ctx.unit_shape, data)
}

fn set_triangle_square_perimeter(data: &mut MazeData) {
    let cols: usize = data.first().unwrap().len();
    let centre_column = cols.div_ceil(2) - 1;
    for (r, row) in data.iter_mut().enumerate() {
        let r_val = ((r + 1) as f32 / 2.).floor() as usize;
        ((centre_column - r_val)..=(centre_column + r_val)).for_each(|c| row[c] = BLOCK);
    }
}

fn set_triangle_hexagon_perimeter(data: &mut MazeData) {
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

fn set_circle_square_perimeter(data: &mut MazeData) {
    let cols: usize = data.first().unwrap().len();
    let centre = cols.div_ceil(2) as isize - 1;
    for (r, row) in data.iter_mut().enumerate() {
        for (c, col) in row.iter_mut().enumerate() {
            let distance = ((r as isize - centre).pow(2) + (c as isize - centre).pow(2)).isqrt();
            if distance <= centre {
                *col = BLOCK;
            }
        }
    }
}

fn set_circle_hexagon_perimeter(data: &mut MazeData, draw_ctx: &DrawContext) {
    let rows: usize = data.len() - 1;
    let cols: usize = data.first().unwrap().len() - 1;
    let centre = (((rows as f32) * draw_ctx.unit_height) / 2., ((cols as f32 - 0.5) * draw_ctx.unit_width) / 2.);
    for (r, row) in data.iter_mut().enumerate() {
        for (c, col) in row.iter_mut().enumerate() {
            let node_pos = if r % 2 == 0 {
                (r as f32 * draw_ctx.unit_height, c as f32 * draw_ctx.unit_width - draw_ctx.unit_width / 2.)
            } else {
                (r as f32 * draw_ctx.unit_height, c as f32 * draw_ctx.unit_width)
            };
            let distance = ((node_pos.0 - centre.0).powf(2.) + (node_pos.1 - centre.1).powf(2.)).sqrt();

            if distance < centre.0 {
                *col = BLOCK;
            }
        }
    }
}

fn set_circle_triangle_perimeter(data: &mut MazeData, draw_ctx: &DrawContext) {
    fn get_centre(v: (Vec2, Vec2, Vec2)) -> (f32, f32) {
        ((v.0.x + v.1.x + v.2.x) / 3., (v.0.y + v.1.y + v.2.y) / 3.)
    }

    let node = Node::new(data.len() - 1, data.first().unwrap().len() - 1);
    let centre = get_centre(draw_ctx.t_vertexes(&node.center()));
    for (r, row) in data.iter_mut().enumerate() {
        for (c, col) in row.iter_mut().enumerate() {
            if let Some(node) = node.at(r, c) {
                let node_pos = get_centre(draw_ctx.t_vertexes(&node));
                let distance = ((node_pos.0 - centre.0).powf(2.) + (node_pos.1 - centre.1).powf(2.)).sqrt();

                if distance < centre.0 {
                    *col = BLOCK;
                }
            }
        }
    }
}
