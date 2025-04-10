use crate::command::{ArgDisplayDensity, ArgDisplaySize};
use amazeing::matrix::{Node, UnitShape};
use std::f32::consts::PI;

#[derive(Debug, Clone)]
pub struct DrawContext {
    pub(crate) margin: f32,
    pub(crate) border: f32,
    pub(crate) size: f32,
    pub(crate) shape: UnitShape,
    pub(crate) height: f32,
    pub(crate) width: f32,
}

impl DrawContext {
    pub fn from(density: ArgDisplayDensity, size: ArgDisplaySize, shape: UnitShape) -> Self {
        let (margin, border, size) = size.size(density.multiplier(), if shape == UnitShape::Hexagon { 0.65 } else { 1.0 });

        Self {
            margin,
            border,
            size,
            shape: shape.clone(),
            height: if shape == UnitShape::Hexagon { ((PI / 6.).sin() + 1.0) * size } else { 0. },
            width: if shape == UnitShape::Hexagon { (PI / 6.).cos() * size * 2.0 } else { 0. },
        }
    }

    pub fn screen_size(&self, rows: usize, cols: usize) -> (u32, u32) {
        match self.shape {
            UnitShape::Square => (
                (self.margin * 2. + cols as f32 * (self.size + self.border)) as u32,
                (self.margin * 2. + rows as f32 * (self.size + self.border)) as u32,
            ),
            UnitShape::Hexagon => {
                let width = (self.margin * 2. + cols as f32 * (self.width + self.border)) as u32 + self.size as u32;
                let height = (self.margin * 2. + rows as f32 * (self.height + self.border)) as u32;
                (width, height)
            }
        }
    }

    pub fn x(&self, node: Node) -> f32 {
        match self.shape {
            UnitShape::Square => self.margin + node.1 as f32 * (self.size + self.border),
            UnitShape::Hexagon => {
                (self.margin + self.size + (node.1 as f32 * self.width) + self.border * node.1 as f32) + self.s(node.0)
            }
        }
    }

    pub fn y(&self, node: Node) -> f32 {
        match self.shape {
            UnitShape::Square => self.margin + node.0 as f32 * (self.size + self.border),
            UnitShape::Hexagon => self.margin + self.size + (node.0 as f32 * self.height) + self.border * node.0 as f32,
        }
    }

    pub fn s(&self, m: usize) -> f32 {
        if m % 2 == 1 { self.width / 2.0 } else { 0. }
    }
}
