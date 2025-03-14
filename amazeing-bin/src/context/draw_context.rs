use crate::command::ArgDisplayDensity;
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

    pub fn size(&mut self, data: (f32, f32, f32, f32)) {
        (self.margin, self.padding, self.cell_width, self.cell_height) = data
    }

    pub fn density(&mut self, density: ArgDisplayDensity) {
        match density {
            ArgDisplayDensity::Connected => self.padding = 0.,
            ArgDisplayDensity::Dense => self.padding = (self.padding / 1.5).ceil(),
            ArgDisplayDensity::Standard => {}
            ArgDisplayDensity::Cozy => self.padding = (self.padding * 1.5).ceil(),
            ArgDisplayDensity::Ample => self.padding = (self.padding * 2.).ceil(),
        }
    }

    pub fn screen_size(&self) -> (u32, u32) {
        (
            (self.margin * 2. + self.maze_cols as f32 * (self.cell_width + self.padding)) as u32,
            (self.margin * 2. + self.maze_rows as f32 * (self.cell_width + self.padding)) as u32,
        )
    }
}

pub static DRAW_CTX: Ctx = LazyLock::new(|| RwLock::new(DrawContext::new()));
