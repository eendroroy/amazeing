use crate::ui::shape::{BORDER, MARGIN, RADIUS, ShapeFactory};
use std::f32::consts::PI;

const SIDES: f32 = 8.;

#[derive(Debug, Copy, Clone)]
pub(crate) struct OctagonShapeFactory {
    pub(crate) margin: f32,
    pub(crate) border: f32,
    pub(crate) radius: f32,
    pub(crate) width: f32,
    pub(crate) height: f32,
}

impl ShapeFactory for OctagonShapeFactory {
    fn new(zoom: f32) -> Self {
        let width = (PI / SIDES).cos() * RADIUS * 2. * zoom;
        let height = (PI / SIDES).cos() * RADIUS * 2. * zoom;

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

    fn xs(&self, _r: usize, _c: usize) -> f32 {
        0.
    }

    fn ys(&self, _r: usize, _c: usize) -> f32 {
        0.
    }
}
