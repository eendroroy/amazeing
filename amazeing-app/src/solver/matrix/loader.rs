use amazeing::solver::matrix::Maze;
use std::fs;
use std::path::Path;

pub(crate) fn loader(path: &str) -> Maze {
    let data = fs::read_to_string(Path::new(path)).expect("Should have been able to read the file");
    let parsed_data = data.split("\n").map(|line| {
        line.split(",").map(|block| {
            block.trim().parse::<u32>().unwrap().into()
        }).collect::<Vec<u32>>()
    }).collect::<Vec<Vec<u32>>>();

    Maze::from(parsed_data)
}
