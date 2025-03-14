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
A maze generator/solver application with simulation/visualization

Usage: amazeing [OPTIONS] <COMMAND>

Commands:
  create  Create a Maze [aliases: C]
  view    View a Maze [aliases: V]
  solve   Solve a Maze [aliases: S]
  help    Print this message or the help of the given subcommand(s)

Options:
      --display-size <SIZE>         Display size [possible values: xxs, xs, s, m, l, xl, xxl]
      --display-density <DENSITY>   Display density [possible values: connected, dense, standard, cozy, ample]
      --color-scheme <SCHEME.TOML>  Color scheme file (.toml) path
  -h, --help                        Print help
  -V, --version                     Print version
```

## Generate Maze

| Algorithm      |                                                                                                            |
|----------------|------------------------------------------------------------------------------------------------------------|
| Randomized Bfs | [![Demo Video](https://img.youtube.com/vi/st8RLTgAuuE/0.jpg)](https://www.youtube.com/watch?v=st8RLTgAuuE) |

## Solve Maze

| Algorithm      |                                                                                                            |
|----------------|------------------------------------------------------------------------------------------------------------|
| DFS            | [![Demo Video](https://img.youtube.com/vi/9F8XRL7lnIU/0.jpg)](https://www.youtube.com/shorts/9F8XRL7lnIU)  |
| BFS            | [![Demo Video](https://img.youtube.com/vi/h8q5vi68fz0/0.jpg)](https://www.youtube.com/shorts/h8q5vi68fz0)  |
| A* (Manhattan) | [![Demo Video](https://img.youtube.com/vi/LkxyikxTX6Y/0.jpg)](https://www.youtube.com/watch?v=LkxyikxTX6Y) |

## License

This project is licensed under the [GNU AGPL-3.0 License](https://www.gnu.org/licenses/agpl-3.0.html). See
the [LICENSE](./LICENSE) file for more details.
