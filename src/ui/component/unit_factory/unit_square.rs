use crate::ui::component::unit_factory::UnitShapeFactory;
use crate::ui::component::{BORDER, MARGIN, RADIUS};
use std::f32::consts::PI;

const SIDES: f32 = 4.;

#[derive(Debug, Copy, Clone)]
pub(crate) struct SquareUnitShapeFactory {
    pub(crate) m: f32,
    pub(crate) b: f32,
    pub(crate) r: f32,
    pub(crate) w: f32,
    pub(crate) h: f32,
}

impl UnitShapeFactory for SquareUnitShapeFactory {
    fn new(zoom: f32) -> Self {
        let w = (PI / SIDES).sin() * RADIUS * 2. * zoom;
        let h = (PI / SIDES).sin() * RADIUS * 2. * zoom;

        Self {
            m: MARGIN * zoom,
            b: BORDER * zoom,
            r: RADIUS * zoom,
            w,
            h,
        }
    }

    fn m(&self) -> f32 {
        self.m
    }

    fn b(&self) -> f32 {
        self.b
    }

    fn r(&self) -> f32 {
        self.r
    }

    fn w(&self) -> f32 {
        self.w
    }

    fn h(&self) -> f32 {
        self.h
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
