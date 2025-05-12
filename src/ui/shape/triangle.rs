use crate::ui::shape::{BORDER, MARGIN, RADIUS, UnitShapeFactory};
use std::f32::consts::PI;

const SIDES: f32 = 3.;

#[derive(Debug, Copy, Clone)]
pub(crate) struct TriangleUnitShapeFactory {
    pub(crate) margin: f32,
    pub(crate) border: f32,
    pub(crate) radius: f32,
    pub(crate) width: f32,
    pub(crate) height: f32,
}

impl UnitShapeFactory for TriangleUnitShapeFactory {
    fn new(zoom: f32) -> Self {
        let width = (PI / SIDES).sin() * RADIUS * 2. * zoom;
        let height = (PI / SIDES).cos() * RADIUS * zoom + RADIUS * zoom;

        Self {
            margin: MARGIN * zoom,
            border: BORDER * zoom,
            radius: RADIUS * zoom,
            width,
            height,
        }
    }

    fn margin(&self) -> f32 {
        self.margin
    }

    fn border(&self) -> f32 {
        self.border
    }

    fn radius(&self) -> f32 {
        self.radius
    }

    fn width(&self) -> f32 {
        self.width
    }

    fn height(&self) -> f32 {
        self.height
    }

    fn sides(&self) -> f32 {
        SIDES
    }

    fn rotation(&self, r: usize, _c: usize) -> f32 {
        if r % 2 == 0 { 90. } else { -90. }
    }

    fn xs(&self, r: usize, _c: usize) -> f32 {
        match r % 4 {
            1 | 2 => (self.width + self.border) / 2.,
            _ => 0.,
        }
    }

    fn ys(&self, r: usize, _c: usize) -> f32 {
        -1. * (r / 2) as f32 * (self.height + self.border)
            + match r % 2 {
                0 => -(self.height - self.radius - self.border) / 2.,
                1 => (self.height - self.radius) / 2. - self.height,
                _ => 0.,
            }
    }

    fn dimension(&self, rows: usize, cols: usize) -> (u32, u32) {
        (
            (self.margin() * 2.0
                + cols as f32 * self.width()
                + (cols - 1) as f32 * self.border()
                + (self.width + self.border) / 2.0) as u32,
            (self.margin() * 2.0 + rows as f32 * self.height() / 2.0 + (rows / 2) as f32 * self.border()) as u32,
        )
    }
}
