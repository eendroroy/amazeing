use amazeing::tiled::Maze;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ViewContext {
    pub(crate) maze_file_path: PathBuf,
    pub(crate) maze: Maze,
}
