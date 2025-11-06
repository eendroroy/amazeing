use crate::core::tiled::{Maze, MazeData, NodeFactory, UnitShape};
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
