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

#### amazeing

```txt
A maze generator/solver application with simulation/visualization

Usage: amazeing [OPTIONS] <COMMAND>

Commands:
  generate   Generate a Maze
  visualize  Visualize a Maze
  modify     Modify a Maze
  simulate   Simulation of Maze solver
  realtime   Realtime path find in a Maze
  help       Print this message or the help of the given subcommand(s)

Options:
      --display-size <SIZE>         Display size [possible values: xxs, xs, s, m, l, xl, xxl]
      --display-scale <SCALE>       Display scale (display size multiplier)
      --color-scheme <SCHEME.TOML>  Color scheme file (.toml) path
  -h, --help                        Print help
  -V, --version                     Print version
```

#### generate

```txt
Generate a Maze

Usage: amazeing generate --maze <MAZE> --procedure <PROCEDURE> --rows <ROWS> --cols <COLS>

Options:
      --maze <MAZE>            File path to dump Maze data
      --procedure <PROCEDURE>  Maze Generation Procedure [possible values: random, dfs]
      --rows <ROWS>            Number of rows
      --cols <COLS>            Number of cols
  -h, --help                   Print help
```

#### visualize

```txt
Visualize a Maze

Usage: amazeing visualize --maze <MAZE>

Options:
      --maze <MAZE>  Maze file path
  -h, --help         Print help
```

#### modify

```txt
Modify a Maze ( Click MouseLeft a cell to open path and Click Shift+MouseLeft a cell to block path )

Usage: amazeing modify --maze <MAZE>

Options:
      --maze <MAZE>  Maze file path
  -h, --help         Print help
```

#### simulate

```txt
Simulation of Maze solver

Usage: amazeing simulate [OPTIONS] --maze <MAZE> --source <USIZE,USIZE> --destination <USIZE,USIZE> --procedure <PROCEDURE>

Options:
      --maze <MAZE>                Maze file path
      --source <USIZE,USIZE>       Source node to start simulation
      --destination <USIZE,USIZE>  Destination node to stop simulation
      --procedure <PROCEDURE>      Maze Solving Procedure [possible values: bfs, dfs, dijkstra, a-star]
      --heuristic <HEURISTIC>      Heuristic function (to use with AStar) [possible values: manhattan, euclidean, chebyshev, octile, dijkstra]
      --fps <FPS>                  Frame update rate
  -h, --help                       Print help
```

#### realtime

```txt
Realtime path finding in a Maze ( Click "MouseLeft" on a cell select source and Click "Shift+MouseLeft" on a cell select destination )

Usage: amazeing realtime [OPTIONS] --maze <MAZE> --procedure <PROCEDURE>

Options:
      --maze <MAZE>            Maze file path
      --procedure <PROCEDURE>  Maze Solving Procedure [possible values: bfs, dfs, dijkstra, a-star]
      --heuristic <HEURISTIC>  Heuristic function (to use with AStar) [possible values: manhattan, euclidean, chebyshev, octile, dijkstra]
  -h, --help                   Print help
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
