use crate::square::cli_viz::VizType::{BLOCK, OPEN, PATH};
use amazeing_core::square::Maze;

pub enum VizType {
    OPEN,
    BLOCK,
    PATH,
}

#[derive(Debug, Clone)]
pub struct CliViz<const ROWS: usize, const COLS: usize = ROWS> {
    pub(crate) data: [[String; COLS]; ROWS],
    pub(crate) open: String,
    pub(crate) path: String,
}

impl<const ROWS: usize, const COLS: usize> CliViz<ROWS, COLS> {
    pub fn from(open: char, block: char, path: char, f: fn(char, VizType) -> String) -> Self {
        Self {
            data: core::array::from_fn(|_| core::array::from_fn(|_| f(block, BLOCK))),
            open: f(open, OPEN),
            path: f(path, PATH),
        }
    }

    pub fn from_maze(
        maze: &Maze<ROWS, COLS>,
        open: char,
        block: char,
        path: char,
        f: fn(char, VizType) -> String,
    ) -> Self {
        let mut vis = Self::from(open, block, path, f);
        vis.merge_maze(maze);
        vis
    }

    pub fn merge_maze(&mut self, maze: &Maze<ROWS, COLS>) {
        for i in 0..ROWS {
            for j in 0..COLS {
                if maze[(i, j)] > 0 {
                    self.data[i][j] = self.open.clone()
                }
            }
        }
    }

    pub fn merged_path(&mut self, path: Vec<(usize, usize)>) -> Self {
        let mut copy = self.clone();

        for p in path {
            copy.data[p.0][p.1] = self.path.clone()
        }

        copy
    }
}

impl<const ROWS: usize, const COLS: usize> std::fmt::Display for CliViz<ROWS, COLS> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> core::fmt::Result {
        writeln!(f).expect("Failed to write to formatter");
        for i in 0..ROWS {
            for j in 0..COLS {
                write!(f, "{}", self.data[i][j]).expect("Failed to write to formatter");
            }
            writeln!(f).expect("Failed to write to formatter");
        }
        Ok(())
    }
}
