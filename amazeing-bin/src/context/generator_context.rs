use crate::context::common::display_size;

#[derive(Debug, Clone)]
pub struct GeneratorContext {
    pub(crate) maze_file_path: String,
    pub(crate) rows: usize,
    pub(crate) cols: usize,
    pub(crate) display_size: String,
}

impl GeneratorContext {
    pub fn new() -> Self {
        Self {
            maze_file_path: String::new(),
            rows: 10usize,
            cols: 10usize,
            display_size: String::from("medium"),
        }
    }

    pub fn display_size(&self) -> (f32, f32, f32, f32) {
        display_size(&*self.display_size)
    }
}
