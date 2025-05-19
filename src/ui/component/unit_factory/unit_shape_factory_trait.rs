use crate::ui::component::helper::create_mesh;
use macroquad::color::Color;
use macroquad::models::Mesh;

#[allow(dead_code)]
pub(crate) trait UnitShapeFactory: Send + Sync {
    fn new(zoom: f32) -> Self
    where
        Self: Sized;

    fn m(&self) -> f32;
    fn b(&self) -> f32;
    fn r(&self) -> f32;
    fn w(&self) -> f32;
    fn h(&self) -> f32;

    fn mbr(&self) -> (f32, f32, f32) {
        (self.m(), self.b(), self.r())
    }

    fn wh(&self) -> (f32, f32) {
        (self.w(), self.h())
    }

    fn sides(&self, r: usize, c: usize) -> f32;
    fn rotation(&self, r: usize, c: usize) -> f32;

    fn xs(&self, r: usize, c: usize) -> f32;
    fn ys(&self, r: usize, c: usize) -> f32;
    fn xys(&self, r: usize, c: usize) -> (f32, f32) {
        (self.xs(r, c), self.ys(r, c))
    }

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

    fn shape(&self, r: usize, c: usize, _rows: usize, _cols: usize, color: Color) -> Mesh {
        create_mesh(self.mbr(), self.wh(), self.sides(r, c) as u8, self.rotation(r, c), (r, c), self.xys(r, c), color)
    }
}
