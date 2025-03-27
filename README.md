# amazeing

Amazeing is a maze generator/solver application with simulation/visualization.

## Installation

Clone the repository and build the project:

```sh
git clone https://github.com/eendroroy/amazeing.git
cd amazeing
cargo install --path amazeing
```

## Usage

```txt
A maze generator/solver application with simulation/visualization

Usage: amazeing [OPTIONS] <COMMAND>

Commands:
  create  Create a Maze [aliases: C]
  view    View a Maze [aliases: V]
  solve   Solve a Maze [aliases: S]
  help    Print this message or the help of the given subcommand(s)

Options:
  -S, --display-size <SIZE>         Display size [possible values: xxs, xs, s, m, l, xl, xxl]
  -D, --display-density <DENSITY>   Display density [possible values: connected, dense, standard, cozy, ample]
  -C, --color-scheme <SCHEME.TOML>  Color scheme file (.toml) path
  -h, --help                        Print help
  -V, --version                     Print version
```

## Example Commands

#### Generate - single source

```shell
# generate 21x21 maze using BFS - single source
amazeing create --maze bfs_21_21.maze --rows 21 --cols 21 --source 10,10 --procedure bfs
# short
amazeing C -m bfs_21_21.maze -r 21 -c 21 -s 10,10 -p bfs

# simulate - multi source
amazeing create --rows 21 --cols 21 --source 10,10 --procedure bfs --verbose --tempo 25
# short
amazeing C -r 21 -c 21 -s 10,10 -p bfs -v -t 25
```

#### Generate - multi source

```shell
# generate 21x21 maze using BFS - multi source
amazeing create --maze bfs_21_21.maze --rows 21 --cols 21 --source 7,7 --source 14,14 --procedure bfs
# short
amazeing C -m bfs_21_21.maze -r 21 -c 21 -s 7,7 -s 14,14 -p bfs

# simulate - multi source
amazeing create --rows 21 --cols 21 --source 7,7 --source 14,14 --procedure bfs --verbose --tempo 25
# short
amazeing C -r 21 -c 21 -s 7,7 -s 14,14 -p bfs -v -t 25
```

#### Update

```shell
# update bfs_21_21.maze
amazeing view --maze bfs_21_21.maze --update
# short
amazeing V -m bfs_21_21.maze -u
```

#### Solve

```shell
# solve bfs_21_21.maze
amazeing solve --maze bfs_21_21.maze --procedure bfs
# short
amazeing S -m bfs_21_21.maze -p bfs

# simulate
amazeing solve --maze bfs_21_21.maze --procedure bfs --verbose --tempo 25
# short
amazeing S -m bfs_21_21.maze -p bfs -v -t 25
```

## Generate Maze

| Algorithm |                                                                                                            |
|-----------|------------------------------------------------------------------------------------------------------------|
| Dfs       | [![Demo Video](https://img.youtube.com/vi/iyxUARc2T2g/0.jpg)](https://www.youtube.com/watch?v=iyxUARc2T2g) |
| Bfs       | [![Demo Video](https://img.youtube.com/vi/st8RLTgAuuE/0.jpg)](https://www.youtube.com/watch?v=st8RLTgAuuE) |

## Solve Maze

| Algorithm      |                                                                                                            |
|----------------|------------------------------------------------------------------------------------------------------------|
| DFS            | [![Demo Video](https://img.youtube.com/vi/9F8XRL7lnIU/0.jpg)](https://www.youtube.com/shorts/9F8XRL7lnIU)  |
| BFS            | [![Demo Video](https://img.youtube.com/vi/h8q5vi68fz0/0.jpg)](https://www.youtube.com/shorts/h8q5vi68fz0)  |
| A* (Manhattan) | [![Demo Video](https://img.youtube.com/vi/LkxyikxTX6Y/0.jpg)](https://www.youtube.com/watch?v=LkxyikxTX6Y) |

## License

This project is licensed under the [GNU AGPL-3.0 License](https://www.gnu.org/licenses/agpl-3.0.html). See
the [LICENSE](./LICENSE) file for more details.
