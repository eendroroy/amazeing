use crate::core::tiled::{MazeShape, UnitShape};
use std::f32::consts::PI;

#[derive(Debug, Clone)]
pub struct DrawContext {
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
        let (unit_height, unit_width) = unit_dimension(unit_shape, zoom * 15. * shape_multiplier);

        Self {
            zoom,
            maze_shape,
            unit_shape,
            unit_height,
            unit_width,
            fps,
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
