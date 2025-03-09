#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Maze {
    pub(crate) data: Vec<Vec<u32>>,
}

impl Maze {
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn from(data: Vec<Vec<u32>>) -> Self {
        Self { data }
    }

    pub fn set_data(&mut self, data: Vec<Vec<u32>>) {
        self.data = data
    }

    pub fn rows(&self) -> usize {
        self.data.iter().len()
    }

    pub fn cols(&self) -> usize {
        self.data.get(0).unwrap().iter().len()
    }
}
