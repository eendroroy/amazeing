use crate::_lib::tiled::Maze;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ViewContext {
    pub(crate) maze_file_path: PathBuf,
    pub(crate) maze: Maze,
}

impl ViewContext {
    pub fn from(maze_file_path: PathBuf, maze: Maze) -> Self {
        Self { maze_file_path, maze }
    }
}
