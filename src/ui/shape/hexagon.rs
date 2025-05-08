use crate::ui::shape::{BORDER, MARGIN, RADIUS, ShapeFactory};
use std::f32::consts::PI;

const SIDES: f32 = 6.;

#[derive(Debug, Copy, Clone)]
pub(crate) struct HexagonShapeFactory {
    pub(crate) margin: f32,
    pub(crate) border: f32,
    pub(crate) radius: f32,
    pub(crate) width: f32,
    pub(crate) height: f32,
}

impl ShapeFactory for HexagonShapeFactory {
    fn new(zoom: f32) -> Self {
        let width = (PI / SIDES).cos() * RADIUS * zoom * 2.;
        let height = RADIUS * zoom * (1. + (PI / SIDES).sin());
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

    fn rotation(&self, _r: usize, _c: usize) -> f32 {
        180. / SIDES
    }

    fn xs(&self, r: usize, _c: usize) -> f32 {
        if r % 2 == 1 { (self.width + self.border) / 2. } else { 0. }
    }

    fn ys(&self, _r: usize, _c: usize) -> f32 {
        self.radius * (1. - (PI / SIDES).sin()) / 2.
    }

    fn dimension(&self, rows: usize, cols: usize) -> (u32, u32) {
        (
            (2. * self.margin()
                + cols as f32 * self.width()
                + (cols - 1) as f32 * self.border()
                + (self.width() + self.border) / 2.) as u32,
            (2. * self.margin()
                + rows as f32 * self.height()
                + (rows - 1) as f32 * self.border()
                + (self.radius * (1. - (PI / SIDES).sin()))) as u32,
        )
    }
}
