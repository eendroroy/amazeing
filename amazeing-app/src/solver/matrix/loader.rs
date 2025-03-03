use amazeing::solver::matrix::Maze;
use std::fs;
use std::path::Path;

pub(crate) fn loader_maze_from_file(path: &str) -> Maze {
    let data = fs::read_to_string(Path::new(path)).expect("Should have been able to read the file");
    let parsed_data = data
        .split("\n")
        .map(|line| {
            line.split(" ")
                .map(|block| block.trim().parse::<u32>().unwrap().into())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    Maze::from(parsed_data)
}

pub(crate) fn parse_node(node: &str) -> (usize, usize) {
    let node_data = node.split(",").collect::<Vec<&str>>();
    (
        u32::from_str_radix(node_data.get(0).unwrap(), 10).unwrap() as usize,
        u32::from_str_radix(node_data.get(1).unwrap(), 10).unwrap() as usize,
    )
}
