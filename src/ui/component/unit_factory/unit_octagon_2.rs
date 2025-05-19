use crate::ui::component::helper::create_mesh;
use crate::ui::component::unit_factory::UnitShapeFactory;
use crate::ui::component::{BORDER, MARGIN, RADIUS};
use crate::utility::IsDivisible;
use macroquad::color::Color;
use macroquad::models::Mesh;
use std::f32::consts::PI;

const SIDES_O: f32 = 8.;
const SIDES_S: f32 = 4.;

#[derive(Debug, Copy, Clone)]
pub(crate) struct Octagon2UnitShapeFactory {
    pub(crate) m: f32,
    pub(crate) b: f32,
    pub(crate) r_o: f32,
    pub(crate) r_s: f32,
    pub(crate) w: f32,
    pub(crate) h: f32,
}

impl UnitShapeFactory for Octagon2UnitShapeFactory {
    fn new(zoom: f32) -> Self {
        let d = (PI / SIDES_O).cos() * RADIUS * 2. * zoom;
        let r_s = RADIUS * zoom - (PI / SIDES_O).sin() * RADIUS * zoom - BORDER * zoom / 2.;

        Self {
            m: MARGIN * zoom,
            b: BORDER * zoom,
            r_o: RADIUS * zoom,
            r_s,
            w: d,
            h: d,
        }
    }

    fn m(&self) -> f32 {
        self.m
    }

    fn b(&self) -> f32 {
        self.b
    }

    fn r(&self) -> f32 {
        self.r_o
    }

    fn w(&self) -> f32 {
        self.w
    }

    fn h(&self) -> f32 {
        self.h
    }

    fn sides(&self, r: usize, _c: usize) -> f32 {
        if r.is_even() { SIDES_O } else { SIDES_S }
    }

    fn rotation(&self, r: usize, _c: usize) -> f32 {
        if r.is_even() { 180. / SIDES_O } else { 0. }
    }

    fn xs(&self, r: usize, _c: usize) -> f32 {
        if r.is_odd() { (self.r_o - self.r_s) * 2. } else { 0. }
    }

    fn ys(&self, r: usize, _c: usize) -> f32 {
        -(self.h + self.b) * (r / 2) as f32 + if r.is_odd() { -(self.h + self.b) / 2. } else { 0.0 }
    }

    fn inner_dimension(&self, rows: usize, cols: usize) -> (u32, u32) {
        (
            (cols as f32 * self.w() + (cols - 1) as f32 * self.b()) as u32,
            (rows.div_ceil(2) as f32 * self.h() + (rows.div_ceil(2) - 1) as f32 * self.b()) as u32,
        )
    }

    fn shape(&self, r: usize, c: usize, _rows: usize, _cols: usize, color: Color) -> Mesh {
        create_mesh(
            if r.is_even() { self.mbr() } else { (self.m, self.b, self.r_s) },
            self.wh(),
            self.sides(r, c) as u8,
            self.rotation(r, c),
            (r, c),
            self.xys(r, c),
            color,
        )
    }
}
