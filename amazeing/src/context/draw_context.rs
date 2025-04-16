use amazeing::matrix::{Node, UnitShape};
use macroquad::prelude::Vec2;
use std::f32::consts::PI;

#[derive(Debug, Clone)]
pub struct DrawContext {
    pub(crate) margin: f32,
    pub(crate) border: f32,
    pub(crate) size: f32,
    pub(crate) u_shape: UnitShape,
    pub(crate) u_height: f32,
    pub(crate) u_width: f32,
    pub(crate) fps: u8,
}

impl DrawContext {
    pub fn from(zoom: f32, unit_shape: UnitShape, fps: u8) -> Self {
        let shape_multiplier = match unit_shape {
            UnitShape::Hexagon | UnitShape::Circle => 0.65,
            _ => 1.0,
        };

        let (margin, border, size) = (zoom * 8., zoom * 3., zoom * 15. * shape_multiplier);

        let (unit_height, unit_width) = match unit_shape {
            UnitShape::Triangle => (size * 0.5 * (PI / 3.).tan(), size),
            UnitShape::Hexagon | UnitShape::Circle => (((PI / 6.).sin() + 1.0) * size, (PI / 6.).cos() * size * 2.0),
            UnitShape::Square => (size, size),
        };

        Self {
            margin,
            border,
            size,
            u_shape: unit_shape.clone(),
            u_height: unit_height,
            u_width: unit_width,
            fps,
        }
    }

    pub fn screen_size(&self, rows: usize, cols: usize) -> (u32, u32) {
        match self.u_shape {
            UnitShape::Triangle => (
                (self.margin * 2. + cols as f32 * (self.u_width + self.border) + self.size / 2.) as u32,
                (self.margin * 2. + rows as f32 * (self.u_height + self.border) / 2.) as u32,
            ),
            UnitShape::Square => (
                (self.margin * 2. + cols as f32 * (self.size + self.border)) as u32,
                (self.margin * 2. + rows as f32 * (self.size + self.border)) as u32,
            ),
            UnitShape::Hexagon | UnitShape::Circle => {
                let width = (self.margin * 2. + cols as f32 * (self.u_width + self.border)) as u32 + self.size as u32;
                let height = (self.margin * 2. + rows as f32 * (self.u_height + self.border)) as u32;
                (width, height)
            }
        }
    }

    pub fn x(&self, node: Node) -> f32 {
        match self.u_shape {
            UnitShape::Triangle => panic!("method x() not applicable for triangular shape"),
            UnitShape::Square => self.margin + node.1 as f32 * (self.size + self.border),
            UnitShape::Hexagon | UnitShape::Circle => {
                (self.margin + self.size + (node.1 as f32 * self.u_width) + self.border * node.1 as f32)
                    + self.s(node.0)
            }
        }
    }

    pub fn y(&self, node: Node) -> f32 {
        match self.u_shape {
            UnitShape::Triangle => panic!("method x() not applicable for triangular shape"),
            UnitShape::Square => self.margin + node.0 as f32 * (self.size + self.border),
            UnitShape::Hexagon | UnitShape::Circle => {
                self.margin + self.size + (node.0 as f32 * self.u_height) + self.border * node.0 as f32
            }
        }
    }

    pub fn s(&self, m: usize) -> f32 {
        if m % 2 == 1 { (self.u_width + self.border) / 2.0 } else { 0. }
    }

    pub fn t_vertexes(&self, node: Node) -> (Vec2, Vec2, Vec2) {
        let base_v1 = Vec2::new(
            self.margin + (node.1 as f32 * (self.u_width + self.border)),
            self.margin + ((node.0 as f32 / 2.).floor() * (self.u_height + self.border)),
        );
        match node.0 % 4 {
            0 => {
                let v1 = Vec2::new(base_v1.x, base_v1.y);
                let v2 = Vec2::new(v1.x + self.u_width, v1.y);
                let v3 = Vec2::new(v1.x + self.u_width / 2., v2.y + self.u_height);

                (v1, v2, v3)
            }
            1 => {
                let v1 = Vec2::new(base_v1.x + self.u_width + self.border / 2., base_v1.y + self.border / 2.);
                let v2 = Vec2::new(v1.x - self.u_width / 2., v1.y + self.u_height);
                let v3 = Vec2::new(v1.x + self.u_width / 2., v1.y + self.u_height);

                (v1, v2, v3)
            }
            2 => {
                let v1 = Vec2::new(base_v1.x + (self.u_width + self.border) / 2., base_v1.y);
                let v2 = Vec2::new(v1.x + self.u_width, v1.y);
                let v3 = Vec2::new(v1.x + self.u_width / 2., v2.y + self.u_height);

                (v1, v2, v3)
            }
            3 => {
                let v1 = Vec2::new(base_v1.x + self.u_width / 2., base_v1.y + self.border / 2.);
                let v2 = Vec2::new(v1.x - self.u_width / 2., v1.y + self.u_height);
                let v3 = Vec2::new(v1.x + self.u_width / 2., v1.y + self.u_height);

                (v1, v2, v3)
            }
            _ => unreachable!(),
        }
    }
}
