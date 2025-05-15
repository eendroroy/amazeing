use crate::ui::component::helper::create_mesh;
use macroquad::color::Color;
use macroquad::models::Mesh;

pub(crate) trait UnitShapeFactory: Send + Sync {
    fn new(zoom: f32) -> Self
    where
        Self: Sized;

    fn m(&self) -> f32;
    fn b(&self) -> f32;
    fn r(&self) -> f32;
    fn w(&self) -> f32;
    fn h(&self) -> f32;

    fn sides(&self) -> f32;
    fn rotation(&self, r: usize, c: usize) -> f32;

    fn xs(&self, r: usize, c: usize) -> f32;
    fn ys(&self, r: usize, c: usize) -> f32;

    fn inner_dimension(&self, rows: usize, cols: usize) -> (u32, u32) {
        (
            (cols as f32 * self.w() + (cols - 1) as f32 * self.b()) as u32,
            (rows as f32 * self.h() + (rows - 1) as f32 * self.b()) as u32,
        )
    }

    fn wh_for(&self, rows: usize, cols: usize) -> (u32, u32) {
        let inner_dimension = self.inner_dimension(rows, cols);
        ((2. * self.m() + inner_dimension.0 as f32) as u32, (2. * self.m() + inner_dimension.1 as f32) as u32)
    }

    fn shape(&self, r: usize, c: usize, color: Color) -> Mesh {
        create_mesh(
            (self.m(), self.b(), self.r()),
            (self.w(), self.h()),
            self.sides() as u8,
            self.rotation(r, c),
            (r, c),
            (self.xs(r, c), self.ys(r, c)),
            color,
        )
    }
}
