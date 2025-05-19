use crate::ui::component::unit_factory::UnitShapeFactory;
use crate::ui::component::{BORDER, MARGIN, RADIUS};
use crate::utility::IsDivisible;
use std::f32::consts::PI;

const SIDES: f32 = 3.;

#[derive(Debug, Copy, Clone)]
pub(crate) struct TriangleUnitShapeFactory {
    pub(crate) m: f32,
    pub(crate) b: f32,
    pub(crate) r: f32,
    pub(crate) w: f32,
    pub(crate) h: f32,
}

impl UnitShapeFactory for TriangleUnitShapeFactory {
    fn new(zoom: f32) -> Self {
        Self {
            m: MARGIN * zoom,
            b: BORDER * zoom,
            r: RADIUS * zoom,
            w: (PI / SIDES).sin() * RADIUS * 2. * zoom,
            h: (PI / SIDES).cos() * RADIUS * zoom + RADIUS * zoom,
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

    fn sides(&self, _r: usize, _c: usize) -> f32 {
        SIDES
    }

    fn rotation(&self, r: usize, _c: usize) -> f32 {
        if r % 2 == 0 { 90. } else { -90. }
    }

    fn xs(&self, r: usize, _c: usize) -> f32 {
        match r % 4 {
            1 | 2 => (self.w + self.b) / 2.,
            _ => 0.,
        }
    }

    fn ys(&self, r: usize, _c: usize) -> f32 {
        -1. * (r / 2) as f32 * (self.h + self.b)
            + match r.is_even() {
                true => -(self.h - self.r - self.b) / 2.,
                false => (self.h - self.r) / 2. - self.h,
            }
    }

    fn inner_dimension(&self, rows: usize, cols: usize) -> (u32, u32) {
        (
            (cols as f32 * self.w() + (cols - 1) as f32 * self.b() + (self.w + self.b) / 2.0) as u32,
            (rows as f32 * self.h() / 2.0 + (rows / 2) as f32 * self.b()) as u32,
        )
    }
}
