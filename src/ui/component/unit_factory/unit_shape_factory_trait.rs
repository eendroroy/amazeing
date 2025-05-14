use crate::ui::component::helper::create_mesh;
use macroquad::color::Color;
use macroquad::models::Mesh;

pub(crate) trait UnitShapeFactory: Send + Sync {
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

    fn inner_dimension(&self, rows: usize, cols: usize) -> (u32, u32) {
        (
            (cols as f32 * self.width() + (cols - 1) as f32 * self.border()) as u32,
            (rows as f32 * self.height() + (rows - 1) as f32 * self.border()) as u32,
        )
    }

    fn dimension(&self, rows: usize, cols: usize) -> (u32, u32) {
        let inner_dimension = self.inner_dimension(rows, cols);
        ((2. * self.margin() + inner_dimension.0 as f32) as u32, (2. * self.margin() + inner_dimension.1 as f32) as u32)
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
