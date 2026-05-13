use crate::maze::{Maze, MazeData, NodeFactory, UnitShape};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;

pub(crate) fn dump_maze_to_file(path: &Path, maze: &Maze) {
    if fs::exists(path).unwrap_or(false) {
        fs::remove_file(path).unwrap();
    }

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .expect("Could not open file");

    file.write_all(format!("{}\n", maze.unit_shape).as_bytes())
        .expect("Could not write to file");

    let node_factory = NodeFactory::new(maze.rows(), maze.cols());

    for r in 0..maze.rows() {
        for c in 0..maze.cols() {
            if let Some(node) = node_factory.at(r, c) {
                file.write_all(format!("{: >2}", maze[node]).as_bytes())
                    .expect("Could not write to file");
                if c < maze.cols() - 1 {
                    file.write_all(" ".as_bytes()).expect("Could not write to file");
                }
            }
        }
        if r < maze.rows() - 1 {
            file.write_all("\n".as_bytes()).expect("Could not write to file");
        }
    }
}

pub(crate) fn load_maze_from_file(path: &Path) -> Maze {
    if !fs::exists(path).unwrap() {
        panic!("Maze file {} does not exists", path.display());
    }

    let file_data = fs::read_to_string(path).expect("Could not read maze file");
    let mut lines = file_data.split("\n").collect::<Vec<&str>>();

    Maze::from(
        UnitShape::from_str(lines.remove(0)).unwrap_or_else(|e| panic!("{}", e)),
        lines
            .iter()
            .map(|line| {
                line.split(" ")
                    .filter(|&x| !x.is_empty())
                    .map(|unit| unit.trim().parse::<i8>().unwrap())
                    .collect::<Vec<i8>>()
            })
            .collect::<MazeData>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::maze::{Maze, NodeFactory, OPEN, UnitShape, VOID};

    fn temp_path(name: &str) -> std::path::PathBuf {
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("{}_{}.maze", name, ts))
    }

    #[test]
    fn dump_and_load_roundtrip_preserves_maze() {
        let mut maze = Maze::new(UnitShape::Square, 2, 3, VOID);
        let f = NodeFactory::new(2, 3);
        maze[f.at(0, 0).unwrap()] = OPEN;
        maze[f.at(1, 2).unwrap()] = OPEN;

        let path = temp_path("amazeing_roundtrip");
        dump_maze_to_file(&path, &maze);
        let loaded = load_maze_from_file(&path);

        assert_eq!(loaded.unit_shape, maze.unit_shape);
        assert_eq!(loaded.data, maze.data);

        let _ = fs::remove_file(path);
    }

    #[test]
    #[should_panic]
    fn load_panics_for_missing_file() {
        let path = temp_path("amazeing_missing");
        load_maze_from_file(&path);
    }
}
