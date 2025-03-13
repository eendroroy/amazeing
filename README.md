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
A maze generator/solver application with simulation/visualization.

amazeing (generate | visualize | solve)

Usage: amazeing [OPTIONS] <COMMAND>

Commands:
  generate   Generate a Maze
  visualize  Visualize a Maze
  solve      Solve a Maze
  help       Print this message or the help of the given subcommand(s)

Options:
      --display-size <SIZE>
          Display size

          [possible values: xxs, xs, s, m, l, xl, xxl]

      --display-scale <SCALE>
          Display scale (display size multiplier)

      --color-scheme <SCHEME.TOML>
          Color scheme file (.toml) path

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
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
