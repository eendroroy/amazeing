use std::sync::{LazyLock, RwLock};

type Ctx = LazyLock<RwLock<DrawContext>>;

#[derive(Debug, Clone)]
pub struct DrawContext {
    pub(crate) maze_rows: usize,
    pub(crate) maze_cols: usize,
    pub(crate) margin: f32,
    pub(crate) padding: f32,
    pub(crate) cell_width: f32,
    pub(crate) cell_height: f32,
}

impl DrawContext {
    pub fn new() -> Self {
        Self {
            maze_rows: 0,
            maze_cols: 0,
            margin: 15.,
            padding: 2.,
            cell_width: 15.,
            cell_height: 15.,
        }
    }

    pub fn set(&mut self, data: (f32, f32, f32, f32)) {
        (self.margin, self.padding, self.cell_width, self.cell_height) = data
    }

    pub fn screen_size(&self) -> (f32, f32) {
        (
            self.margin * 2. + self.maze_cols as f32 * (self.cell_width + self.padding),
            self.margin * 2. + self.maze_rows as f32 * (self.cell_width + self.padding),
        )
    }
}

pub static DRAW_CTX: Ctx = LazyLock::new(|| RwLock::new(DrawContext::new()));
