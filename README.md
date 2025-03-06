# amazeing

Amazeing is a maze generator/solver application with simulation/visualization.

## Installation

Clone the repository and build the project:

```sh
git clone https://github.com/eendroroy/amazeing.git
cd amazeing
cargo build --release
cargo install --path amazeing-solver
cargo install --path amazeing-generator
```

## Usage

```txt
Usage: amazeing

Options:
    -h, --help                 Print the help menu
        --path     str         Path to the maze file
        --from     usize,usize Start point
        --to       usize,usize End point
        --algorithm            Algorithm name for simulation
                               Choose from: bfs, dfs, dijkstra, a-star
        --heu      str         Heuristic function to use with a-star
                               Choose from: manhattan, euclidean, chebyshev, octile, dijkstra
                               Default dijkstra if none provided
        --fps      u8          Gui FPS
```

## License

This project is licensed under the [GNU AGPL-3.0 License](https://www.gnu.org/licenses/agpl-3.0.html). See
the [LICENSE](./LICENSE) file for more details.
