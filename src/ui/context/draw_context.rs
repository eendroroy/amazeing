use crate::core::tiled::{MazeShape, Node, UnitShape};
use macroquad::prelude::Vec2;
use std::f32::consts::PI;

#[derive(Debug, Clone)]
pub struct DrawContext {
    pub(crate) margin: f32,
    pub(crate) border: f32,
    pub(crate) zoom: f32,
    pub(crate) maze_shape: MazeShape,
    pub(crate) unit_shape: UnitShape,
    pub(crate) unit_height: f32,
    pub(crate) unit_width: f32,
    pub(crate) fps: u8,
}

impl DrawContext {
    pub fn from(zoom: f32, maze_shape: MazeShape, unit_shape: UnitShape, fps: u8) -> Self {
        let shape_multiplier = match unit_shape {
            UnitShape::Hexagon => 0.65,
            _ => 1.0,
        };
        let (margin, border, size) = (zoom * 15., zoom * 3., zoom * 15. * shape_multiplier);
        let (unit_height, unit_width) = unit_dimension(unit_shape, size);

        Self {
            margin,
            border,
            zoom,
            maze_shape,
            unit_shape,
            unit_height,
            unit_width,
            fps,
        }
    }

    pub fn t_vertexes(&self, node: &Node) -> (Vec2, Vec2, Vec2) {
        let base_v1 = Vec2::new(
            self.margin + (node.col as f32 * (self.unit_width + self.border)),
            self.margin + ((node.row as f32 / 2.).floor() * (self.unit_height + self.border)),
        );
        match node.row % 4 {
            0 => {
                let v1 = Vec2::new(base_v1.x, base_v1.y);
                let v2 = Vec2::new(v1.x + self.unit_width, v1.y);
                let v3 = Vec2::new(v1.x + self.unit_width / 2., v2.y + self.unit_height);

                (v1, v2, v3)
            }
            1 => {
                let v1 = Vec2::new(base_v1.x + self.unit_width + self.border / 2., base_v1.y + self.border / 2.);
                let v2 = Vec2::new(v1.x + self.unit_width / 2., v1.y + self.unit_height);
                let v3 = Vec2::new(v1.x - self.unit_width / 2., v1.y + self.unit_height);

                (v1, v2, v3)
            }
            2 => {
                let v1 = Vec2::new(base_v1.x + (self.unit_width + self.border) / 2., base_v1.y);
                let v2 = Vec2::new(v1.x + self.unit_width, v1.y);
                let v3 = Vec2::new(v1.x + self.unit_width / 2., v2.y + self.unit_height);

                (v1, v2, v3)
            }
            3 => {
                let v1 = Vec2::new(base_v1.x + self.unit_width / 2., base_v1.y + self.border / 2.);
                let v2 = Vec2::new(v1.x + self.unit_width / 2., v1.y + self.unit_height);
                let v3 = Vec2::new(v1.x - self.unit_width / 2., v1.y + self.unit_height);

                (v1, v2, v3)
            }
            _ => unreachable!(),
        }
    }
}

fn unit_dimension(unit_shape: UnitShape, size: f32) -> (f32, f32) {
    match unit_shape {
        UnitShape::Triangle => (size * 0.5 * (PI / 3.).tan(), size),
        UnitShape::Hexagon => (((PI / 6.).sin() + 1.0) * size, (PI / 6.).cos() * size * 2.0),
        UnitShape::Square => (size, size),
        UnitShape::Octagon => (size, size),
    }
}
