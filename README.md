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
Usage: amazeing --generate | --view | --modify | --simulate | --realtime

Options:
        --help                  Print the help menu
        --generate              Generate Maze
        --view                  View Maze
        --modify                Edit Maze
        --simulate              Simulate Maze solution
        --realtime              Solve Maze on the fly
        --display               Set display size
                                Choose from: x-small (xs), small (s), medium (m), large (l), x-large (xl)

Usage: amazeing --generate

Options:
        --maze      str         Path to the file to dump the maze (existing file will preload the data)
        --rows      usize       Number of ROWS in the maze
        --cols      usize       Number of COLS in the maze
        --proc      str         Procedure to generate MAZE

Usage: amazeing --view | --modify

Options:
        --maze      str         Path to the maze file

Usage: amazeing --simulate

Options:
        --maze      str         Path to the maze file
        --source    usize,usize Start point
        --target    usize,usize End point
        --proc                  Procedure name for simulation
                                Choose from: bfs, dfs, dijkstra, a-star
        --heu       str         Heuristic function to use with a-star
                                Choose from: Manhattan, Euclidean, chebyshev, octile, dijkstra
                                Default dijkstra if none provided
        --fps       u8          Frame per second

Usage: amazeing --realtime

Options:
        --maze      str         Path to the maze file
        --proc                  Procedure name for simulation
                                Choose from: bfs, dfs
        --heu       str         Heuristic function to use with a-star
                                Choose from: Manhattan, Euclidean, chebyshev, octile, dijkstra
                                Default dijkstra if none provided
```

Note:

- Press `q` to quit. This will dump the displayed maze into file in generator mode (`--generate`)
- Press `s` in Simulation mode (`--simulate`) to start simulation.

## Demo

## Demo

| Algorithm      |                                                                                                            |
|----------------|------------------------------------------------------------------------------------------------------------|
| DFS            | [![Demo Video](https://img.youtube.com/vi/9F8XRL7lnIU/0.jpg)](https://www.youtube.com/shorts/9F8XRL7lnIU)  |
| BFS            | [![Demo Video](https://img.youtube.com/vi/h8q5vi68fz0/0.jpg)](https://www.youtube.com/shorts/h8q5vi68fz0)  |
| A* (Manhattan) | [![Demo Video](https://img.youtube.com/vi/LkxyikxTX6Y/0.jpg)](https://www.youtube.com/watch?v=LkxyikxTX6Y) |

## License

This project is licensed under the [GNU AGPL-3.0 License](https://www.gnu.org/licenses/agpl-3.0.html). See
the [LICENSE](./LICENSE) file for more details.
