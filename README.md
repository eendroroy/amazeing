# amazeing

Amazeing is a Rust-based maze generator/solver application.

## Installation

Clone the repository and build the project:

```sh
git clone https://github.com/eendroroy/amazeing.git
cd amazeing
cargo build --release
cargo install --path amazeing-app
```

## Usage

```txt
Usage: amezing [options]

Options:
    -h, --help           Print the help menu
        --ui-cli         Run the simulation in cli mode instead of gui
        --bfs            Run the simulation for BFS
        --dfs            Run the simulation for DFS
        --path <str>     Path to the maze file
```

## Sample

| BFS CLI                                                      | DFS CLI                                                      |
|--------------------------------------------------------------|--------------------------------------------------------------|
| `$ amezing --bfs --ui-cli --path assets/maze/M1_R31_C31.txt` | `$ amezing --dfs --ui-cli --path assets/maze/M1_R31_C31.txt` |
| ![bfs_cli.png](assets/images/bfs_cli.png)                    | ![dfs_cli.png](assets/images/dfs_cli.png)                    |

## License

This project is licensed under the [GNU AGPL-3.0 License](https://www.gnu.org/licenses/agpl-3.0.html). See
the [LICENSE](./LICENSE) file for more details.
