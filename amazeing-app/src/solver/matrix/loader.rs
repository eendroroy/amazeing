use amazeing::solver::matrix::Maze;
use std::fs;
use std::path::Path;

pub(crate) fn loader<const ROWS: usize, const COLS: usize>(path: &str) -> Maze<ROWS, COLS> {
    let data = fs::read_to_string(Path::new(path)).expect("Should have been able to read the file");
    let parsed_data = data.split("\n").map(|line| {
        line.split(",").map(|block| {
            block.parse::<u32>().unwrap().into()
        }).collect::<Vec<u32>>().try_into().expect("Incorrect dimensions")
    }).collect::<Vec<[u32; COLS]>>();

    let parsed_array: [[u32; COLS]; ROWS] = parsed_data.try_into().expect("Incorrect dimensions");

    Maze::from(parsed_array)
}
