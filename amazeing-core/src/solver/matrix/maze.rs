#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Maze<const ROWS: usize, const COLS: usize> {
    pub(crate) data: [[u8; COLS]; ROWS],
}

impl<const ROWS: usize, const COLS: usize> Maze<ROWS, COLS> {
    pub fn new() -> Self {
        Self {
            data: std::array::from_fn(|_| std::array::from_fn(|_| 0u8)),
        }
    }

    pub fn from(data: [[u8; COLS]; ROWS]) -> Self {
        Self { data }
    }

    pub fn rows(&self) -> usize {
        ROWS.clone()
    }

    pub fn cols(&self) -> usize {
        COLS.clone()
    }
}
