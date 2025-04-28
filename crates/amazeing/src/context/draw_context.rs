use amazeing::tiled::{MazeShape, Node, UnitShape};
use macroquad::prelude::Vec2;
use std::f32::consts::PI;

#[derive(Debug, Clone)]
pub struct DrawContext {
    pub(crate) margin: f32,
    pub(crate) border: f32,
    pub(crate) size: f32,
    pub(crate) maze_shape: MazeShape,
    pub(crate) unit_shape: UnitShape,
    pub(crate) unit_height: f32,
    pub(crate) unit_width: f32,
    pub(crate) fps: u8,
}

impl DrawContext {
    pub fn from(zoom: f32, maze_shape: MazeShape, unit_shape: UnitShape, fps: u8) -> Self {
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
            maze_shape,
            unit_shape,
            unit_height,
            unit_width,
            fps,
        }
    }

    pub fn screen_size(&self, rows: usize, cols: usize) -> (u32, u32) {
        let (blocks_width, blocks_height) = (
            cols as f32 * (self.unit_width + self.border) - self.border,
            rows as f32 * (self.unit_height + self.border) - self.border,
        );
        match (self.unit_shape, self.maze_shape) {
            (UnitShape::Triangle, _) => {
                let width = (self.margin * 2. + blocks_width + self.size / 2.) as u32;
                let height = (self.margin * 2. + blocks_height / 2.) as u32;
                (width, height)
            }
            (UnitShape::Square, _) | (UnitShape::Hexagon | UnitShape::Circle, MazeShape::Triangle) => {
                let width = (self.margin * 2. + blocks_width) as u32;
                let height = (self.margin * 2. + blocks_height) as u32;
                (width, height)
            }
            (UnitShape::Hexagon | UnitShape::Circle, _) => {
                let width = (self.margin * 2. + blocks_width) as u32 + self.unit_width as u32;
                let height = (self.margin * 2. + blocks_height) as u32;
                (width, height)
            }
        }
    }

    pub fn x(&self, node: Node) -> f32 {
        match self.unit_shape {
            UnitShape::Triangle => panic!("method x() not applicable for triangular shape"),
            UnitShape::Square => self.margin + node.col as f32 * (self.size + self.border),
            UnitShape::Hexagon | UnitShape::Circle => {
                (self.margin + self.size + (node.col as f32 * self.unit_width) + self.border * node.col as f32)
                    + self.s(node.row)
            }
        }
    }

    pub fn y(&self, node: Node) -> f32 {
        match self.unit_shape {
            UnitShape::Triangle => panic!("method x() not applicable for triangular shape"),
            UnitShape::Square => self.margin + node.row as f32 * (self.size + self.border),
            UnitShape::Hexagon | UnitShape::Circle => {
                self.margin + self.size + (node.row as f32 * self.unit_height) + self.border * node.row as f32
            }
        }
    }

    pub fn s(&self, m: usize) -> f32 {
        if m % 2 == 1 { (self.unit_width + self.border) / 2.0 } else { 0. }
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
