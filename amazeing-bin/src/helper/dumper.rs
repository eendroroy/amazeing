use amazeing::maze::matrix::Maze;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

pub(crate) fn dump_maze_to_file(path: &Path, maze: &Maze) {
    if fs::exists(path).unwrap_or(false) {
        fs::remove_file(path).unwrap();
    }

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .expect("Could not open file");

    for r in 0..maze.rows() {
        for c in 0..maze.cols() {
            file.write_all(format!("{}", maze[(r, c)]).to_string().as_bytes())
                .expect("Could not write to file");
            if c < maze.cols() - 1 {
                file.write_all(" ".to_string().as_bytes())
                    .expect("Could not write to file");
            }
        }
        if r < maze.rows() - 1 {
            file.write_all("\n".to_string().as_bytes())
                .expect("Could not write to file");
        }
    }
}
