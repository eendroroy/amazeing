use crate::ui::component::helper::create_mesh;
use crate::ui::component::unit_factory::UnitShapeFactory;
use crate::ui::component::{BORDER, MARGIN, RADIUS};
use crate::utility::IsDivisible;
use macroquad::color::Color;
use macroquad::math::vec2;
use macroquad::models::{Mesh, Vertex};
use macroquad::prelude::vec3;
use std::f32::consts::PI;

const SIDES: f32 = 6.;

#[derive(Debug, Copy, Clone)]
pub(crate) struct HexagonRectangleUnitShapeFactory {
    m: f32,
    b: f32,
    r: f32,
    w: f32,
    h: f32,
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    x3: f32,
    y3: f32,
}

impl UnitShapeFactory for HexagonRectangleUnitShapeFactory {
    fn new(zoom: f32) -> Self {
        let w = (PI / SIDES).cos() * RADIUS * 2. * zoom;

        let (s_w, s_h) = (w, (RADIUS * zoom - (PI / SIDES).sin() * RADIUS * zoom) * 2.);

        let (x0, y0) = (
            MARGIN * zoom + w + BORDER * zoom / 2.,
            MARGIN * zoom + RADIUS * zoom * 2. - s_h / 2. + BORDER * zoom / 2.,
        );
        let (x1, y1) = (x0 + s_w / 2., y0 + s_h / 2.);
        let (x2, y2) = (x0, y0 + s_h);
        let (x3, y3) = (x0 - s_w / 2., y0 + s_h / 2.);

        Self {
            m: MARGIN * zoom,
            b: BORDER * zoom,
            r: RADIUS * zoom,
            w,
            h: RADIUS * zoom * 2.,
            x0,
            x1,
            x2,
            x3,
            y0,
            y1,
            y2,
            y3,
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

    fn rotation(&self, _r: usize, _c: usize) -> f32 {
        180. / SIDES
    }

    fn xs(&self, r: usize, c: usize) -> f32 {
        if r.is_even() { 0. } else { c as f32 * (self.w + self.b) }
    }

    fn ys(&self, r: usize, _c: usize) -> f32 {
        if r.is_even() {
            -(self.h + self.b) * (r / 2) as f32
        } else {
            (self.h + self.b) * (r / 2) as f32
        }
    }

    fn inner_dimension(&self, rows: usize, cols: usize) -> (u32, u32) {
        (
            (cols as f32 * self.w() + (cols - 1) as f32 * self.b()) as u32,
            (rows.div_ceil(2) as f32 * self.h() + (rows.div_ceil(2) - 1) as f32 * self.b()) as u32,
        )
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
            Mesh {
                vertices: vec![
                    Vertex::new2(vec3(self.xs(r, c) + self.x0, self.ys(r, c) + self.y0, 0.), vec2(0., 0.), color),
                    Vertex::new2(vec3(self.xs(r, c) + self.x1, self.ys(r, c) + self.y1, 0.), vec2(0., 0.), color),
                    Vertex::new2(vec3(self.xs(r, c) + self.x2, self.ys(r, c) + self.y2, 0.), vec2(0., 0.), color),
                    Vertex::new2(vec3(self.xs(r, c) + self.x3, self.ys(r, c) + self.y3, 0.), vec2(0., 0.), color),
                ],
                indices: vec![0, 1, 2, 0, 2, 3],
                texture: None,
            }
        }
    }
}
