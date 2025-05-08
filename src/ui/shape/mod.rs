use crate::ui::shape::create_mesh::create_mesh;
use macroquad::color::Color;
use macroquad::models::Mesh;

pub(crate) mod create_mesh;
pub(crate) mod hexagon;
pub(crate) mod maze_mesh;
pub(crate) mod octagon;
pub(crate) mod square;
pub(crate) mod triangle;

pub(crate) trait ShapeFactory: Send + Sync {
    fn new(zoom: f32) -> Self
    where
        Self: Sized;

    fn margin(&self) -> f32;
    fn border(&self) -> f32;
    fn radius(&self) -> f32;
    fn width(&self) -> f32;
    fn height(&self) -> f32;

    fn sides(&self) -> f32;
    fn rotation(&self, r: usize, c: usize) -> f32;

    fn xs(&self, r: usize, c: usize) -> f32;
    fn ys(&self, r: usize, c: usize) -> f32;

    fn dimension(&self, rows: usize, cols: usize) -> (u32, u32) {
        (
            (2. * self.margin() + cols as f32 * self.width() + (cols - 1) as f32 * self.border()) as u32,
            (2. * self.margin() + rows as f32 * self.height() + (rows - 1) as f32 * self.border()) as u32,
        )
    }

    fn shape(&self, r: usize, c: usize, color: Color) -> Mesh {
        create_mesh(
            (self.margin(), self.border(), self.radius()),
            (self.width(), self.height()),
            self.sides() as u8,
            self.rotation(r, c),
            (r, c),
            (self.xs(r, c), self.ys(r, c)),
            color,
        )
    }
}

pub(crate) const MARGIN: f32 = 15.;
pub(crate) const BORDER: f32 = 2.;
pub(crate) const RADIUS: f32 = 25.;
