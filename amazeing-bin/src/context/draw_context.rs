#[derive(Debug, Clone)]
pub struct DrawContext {
    pub(crate) margin: f32,
    pub(crate) padding: f32,
    pub(crate) cell_width: f32,
    pub(crate) cell_height: f32,
}
