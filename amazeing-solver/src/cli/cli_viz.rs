use crate::cli::cli_viz::VizType::{BLOCK, OPEN, PATH};
use amazeing::solver::matrix::Maze;

pub enum VizType {
    OPEN,
    BLOCK,
    PATH,
}

#[derive(Debug, Clone)]
pub struct CliViz {
    pub(crate) data: Vec<Vec<String>>,
    pub(crate) open: String,
    pub(crate) path: String,
}

impl CliViz {
    pub fn from(
        open: char,
        block: char,
        path: char,
        f: fn(char, VizType) -> String,
        rows: usize,
        cols: usize,
    ) -> Self {
        Self {
            data: vec![vec![f(block, BLOCK); cols]; rows],
            open: f(open, OPEN),
            path: f(path, PATH),
        }
    }

    pub fn from_maze(
        maze: &Maze,
        open: char,
        block: char,
        path: char,
        f: fn(char, VizType) -> String,
    ) -> Self {
        let mut vis = Self::from(open, block, path, f, maze.rows(), maze.cols());
        vis.merge_maze(maze);
        vis
    }

    pub fn rows(&self) -> usize {
        self.data.iter().len()
    }

    pub fn cols(&self) -> usize {
        self.data.get(0).unwrap().iter().len()
    }

    pub fn merged_path(&mut self, path: Vec<(usize, usize)>) -> Self {
        let mut copy = self.clone();

        for p in path {
            copy.data[p.0][p.1] = self.path.clone()
        }

        copy
    }

    pub fn merge_maze(&mut self, maze: &Maze) {
        for i in 0..maze.rows() {
            for j in 0..maze.cols() {
                if maze[(i, j)] > 0 {
                    self.data[i][j] = self.open.clone()
                }
            }
        }
    }
}

impl std::fmt::Display for CliViz {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> core::fmt::Result {
        writeln!(f).expect("Failed to write to formatter");
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                write!(f, "{}{}", self.data[i][j], self.data[i][j])
                    .expect("Failed to write to formatter");
            }
            writeln!(f).expect("Failed to write to formatter");
        }
        Ok(())
    }
}
