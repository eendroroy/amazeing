use amazeing::maze::matrix::Maze;
use std::fs;
use std::path::Path;

pub(crate) fn loader_maze_from_file(path: &str) -> Maze {
    if !fs::exists(Path::new(&path)).unwrap() {
        panic!("Maze file {} does not exists", path)
    }
    Maze::from(
        fs::read_to_string(Path::new(path))
            .expect("Should have been able to read the file")
            .split("\n")
            .map(|line| {
                line.split(" ")
                    .map(|block| block.trim().parse::<u32>().unwrap().into())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>(),
    )
}
