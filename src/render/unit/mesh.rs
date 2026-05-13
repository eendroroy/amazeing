use macroquad::color::Color;
use macroquad::math::Vec2;
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

#[allow(dead_code)]
pub(crate) fn create_arc_mesh(
    (x, y): (f32, f32),
    sides: u8,
    radius: f32,
    rotation: f32,
    thickness: f32,
    arc: f32,
    color: Color,
) -> Mesh {
    let rot = rotation.to_radians();
    let part = arc.to_radians();

    let sides = (sides as f32 * part / std::f32::consts::TAU).ceil().max(1.0);
    let span = part / sides;
    let sides = sides as usize;

    let mut vertices = Vec::<Vertex>::with_capacity(sides * 2);
    let mut indices = Vec::<u16>::with_capacity(sides * 2);

    for i in 0..sides {
        let start_angle = i as f32 * span + rot;
        let end_angle = start_angle + span;

        indices.extend([0, 1, 2, 2, 1, 3].map(|k| k + (vertices.len() as u16)));

        for (angle, radius) in [
            (start_angle, radius),
            (start_angle, radius + thickness),
            (end_angle, radius),
            (end_angle, radius + thickness),
        ] {
            let point = Vec2::new(x, y) + radius * Vec2::from_angle(angle);
            vertices.push(Vertex::new(point.x, point.y, 0., 0., 0., color));
        }
    }

    Mesh {
        vertices,
        indices,
        texture: None,
    }
}
