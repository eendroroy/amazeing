use macroquad::color::Color;
use macroquad::models::{Mesh, Vertex};
use std::f32::consts::PI;

pub(crate) fn create_mesh(
    (margin, border, radius): (f32, f32, f32),
    (width, height): (f32, f32),
    sides: u8,
    rotation: f32,
    (r, c): (usize, usize),
    (xs, ys): (f32, f32),
    color: Color,
) -> Mesh {
    let (x0, y0) = (
        margin + c as f32 * (border + width) + width / 2. + xs,
        margin + r as f32 * (border + height) + height / 2. + ys,
    );

    let mut vertices = Vec::<Vertex>::with_capacity(sides as usize);
    let mut indices = vec![];
    for i in 0..sides {
        let rx = (i as f32 / sides as f32 * PI * 2. + rotation.to_radians()).cos();
        let ry = (i as f32 / sides as f32 * PI * 2. + rotation.to_radians()).sin();

        let vertex = Vertex::new(x0 + radius * rx, y0 + radius * ry, 0., rx, ry, color);

        vertices.push(vertex);

        if i < sides - 2 {
            indices.extend_from_slice(&[0, i as u16 + 1, i as u16 + 2]);
        }
    }

    Mesh {
        vertices,
        indices,
        texture: None,
    }
}
