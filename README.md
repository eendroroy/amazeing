# amazeing

Amazeing is a maze generator/solver application with simulation/visualization.

## Installation

Clone the repository and build the project:

```sh
git clone https://github.com/eendroroy/amazeing.git
cd amazeing
cargo build --release
cargo install --path amazeing-bin
```

## Usage

```txt
Usage: amazeing --solve | --generate | --view

Options:
    -h, --help                  Print the help menu
    -s, --solve                 Solve Maze
    -g, --generate              Generate Maze
    -v, --view                  View Maze
        --display               Set display size
                                Choose from: x-small (xs), small (s), medium (m), large (l), x-large (xl)

Usage: amazeing --solve

Options:
        --maze      str         Path to the maze file
        --from      usize,usize Start point
        --to        usize,usize End point
        --algorithm             Algorithm name for simulation
                                Choose from: bfs, dfs, dijkstra, a-star
        --heu       str         Heuristic function to use with a-star
                                Choose from: manhattan, euclidean, chebyshev, octile, dijkstra
                                Default dijkstra if none provided
        --fps       u8          Frame per second

Usage: amazeing --generate

Options:
        --maze      str         Path to the file to dump the maze (existing file will preload the data)
        --rows      usize       Number of ROWS in the maze
        --cols      usize       Number of COLS in the maze
        --proc      str         Procedure to generate MAZE

Usage: amazeing --view

Options:
        --maze      str         Path to the maze file
```

Note:

- Press `q` to quit. This will dump the displayed maze into file in generator mode (`--generate`)
- Press `s` in solver mode (`--solve`) to start simulation.

## Demo

## Demo

| Algorithm |                                                                                                           |
|-----------|-----------------------------------------------------------------------------------------------------------|
| DFS       | [![Demo Video](https://img.youtube.com/vi/9F8XRL7lnIU/0.jpg)](https://www.youtube.com/shorts/9F8XRL7lnIU) |
| BFS       | [![Demo Video](https://img.youtube.com/vi/h8q5vi68fz0/0.jpg)](https://www.youtube.com/shorts/h8q5vi68fz0) |

## License

This project is licensed under the [GNU AGPL-3.0 License](https://www.gnu.org/licenses/agpl-3.0.html). See
the [LICENSE](./LICENSE) file for more details.
