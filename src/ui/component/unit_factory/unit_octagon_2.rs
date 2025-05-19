use crate::ui::component::helper::create_mesh;
use crate::ui::component::unit_factory::UnitShapeFactory;
use crate::ui::component::{BORDER, MARGIN, RADIUS};
use crate::utility::IsDivisible;
use macroquad::color::Color;
use macroquad::models::Mesh;
use std::f32::consts::PI;

const SIDES_ODD: f32 = 8.;
const SIDES_EVEN: f32 = 4.;

#[derive(Debug, Copy, Clone)]
pub(crate) struct Octagon2UnitShapeFactory {
    pub(crate) m: f32,
    pub(crate) b: f32,
    pub(crate) r: f32,
    pub(crate) w_o: f32,
    pub(crate) h_o: f32,
    pub(crate) w_s: f32,
    pub(crate) h_s: f32,
}

impl UnitShapeFactory for Octagon2UnitShapeFactory {
    fn new(zoom: f32) -> Self {
        let d = (PI / SIDES_ODD).cos() * RADIUS * 2. * zoom;

        Self {
            m: MARGIN * zoom,
            b: BORDER * zoom,
            r: RADIUS * zoom,
            w_o: d,
            h_o: d,
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
        self.w_o
    }

    fn h(&self) -> f32 {
        self.h_o
    }

    fn sides(&self, r: usize, _c: usize) -> f32 {
        if r.is_odd() { SIDES_ODD } else { SIDES_EVEN }
    }

    fn rotation(&self, r: usize, _c: usize) -> f32 {
        if r.is_odd() { 180. / SIDES_ODD } else { 0. }
    }

    fn xs(&self, _r: usize, _c: usize) -> f32 {
        0.
    }

    fn ys(&self, _r: usize, _c: usize) -> f32 {
        0.
    }

    fn shape(&self, r: usize, c: usize, _rows: usize, _cols: usize, color: Color) -> Mesh {
        if r.is_even() {
            create_mesh(
                self.mbr(),
                self.wh(),
                self.sides(r, c) as u8,
                self.rotation(r, c),
                (r, c),
                self.xys(r, c),
                color,
            )
        } else {
            create_mesh(
                self.mbr(),
                (self.w_s, self.h_s),
                self.sides(r, c) as u8,
                self.rotation(r, c),
                (r, c),
                self.xys(r, c),
                color,
            )
        }
    }
}
