use amazeing::maze::matrix::Maze;
use amazeing::DNode;
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

pub(crate) fn parse_node(node: &str) -> DNode {
    let node_data = node.split(",").collect::<Vec<&str>>();
    (
        u32::from_str_radix(node_data.get(0).unwrap(), 10).unwrap() as usize,
        u32::from_str_radix(node_data.get(1).unwrap(), 10).unwrap() as usize,
    )
}
