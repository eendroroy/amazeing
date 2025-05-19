use crate::ui::component::unit_factory::UnitShapeFactory;
use crate::ui::component::{BORDER, MARGIN, RADIUS};
use crate::utility::IsEvenOdd;
use std::f32::consts::PI;

const SIDES: f32 = 6.;

#[derive(Debug, Copy, Clone)]
pub(crate) struct HexagonUnitShapeFactory {
    pub(crate) m: f32,
    pub(crate) b: f32,
    pub(crate) r: f32,
    pub(crate) w: f32,
    pub(crate) h: f32,
}

impl UnitShapeFactory for HexagonUnitShapeFactory {
    fn new(zoom: f32) -> Self {
        Self {
            m: MARGIN * zoom,
            b: BORDER * zoom,
            r: RADIUS * zoom,
            w: (PI / SIDES).cos() * RADIUS * zoom * 2.,
            h: RADIUS * zoom * (1. + (PI / SIDES).sin()),
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

    fn xs(&self, r: usize, _c: usize) -> f32 {
        if r.is_odd() { (self.w + self.b) / 2. } else { 0. }
    }

    fn ys(&self, _r: usize, _c: usize) -> f32 {
        self.r * (1. - (PI / SIDES).sin()) / 2.
    }

    fn inner_dimension(&self, rows: usize, cols: usize) -> (u32, u32) {
        (
            (cols as f32 * self.w() + (cols - 1) as f32 * self.b() + (self.w() + self.b) / 2.) as u32,
            (rows as f32 * self.h() + (rows - 1) as f32 * self.b() + (self.r * (1. - (PI / SIDES).sin()))) as u32,
        )
    }
}
