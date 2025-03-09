#[derive(Debug, Clone)]
pub struct GeneratorContext {
    pub(crate) maze_file_path: String,
    pub(crate) rows: usize,
    pub(crate) cols: usize,
}

impl GeneratorContext {
    pub fn new() -> Self {
        Self {
            maze_file_path: String::new(),
            rows: 10usize,
            cols: 10usize,
        }
    }
}
