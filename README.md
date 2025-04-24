# amazeing

Amazeing is a maze generator/solver application with simulation/visualization.

![title_animation_1.gif](assets/image/title_animation_1.gif)
![title_animation_2.gif](assets/image/title_animation_2.gif)
![title_animation_3.gif](assets/image/title_animation_3.gif)

## Installation

Clone the repository and build the project:

```sh
git clone https://github.com/eendroroy/amazeing.git
cd amazeing
make install # this will install shell-completions
```

## Usage

> Run `amazeing --help`

### Key/Mouse events

- `Control`+`I` - Save current screen as png in current directory
- Create
    - `G`/`<Space>` - (Re)Generate Maze
    - Additional action in simulation mode (`--verbose`/`-v`):
        - `LeftClick` - Add Source
        - `Shift`+`LeftClick` - Remove Source
        - `S`/`<Space>` - Start Simulation
        - `<Space>` - Start/Pause/Resume Simulation

- View - Update mode (`--update`/`-u`):
    - `LeftClick` - Open path
    - `Shift`+`LeftClick` - Block Path
    - `Control`+`S` - Save current maze to the same file

- Solve
    - `LeftClick` - Select Source
    - `Shift`+`LeftClick` - Select Destination
    - Additional action in simulation mode (`--verbose`/`-v`):
        - `S`/`<Space>` - Start Simulation
        - `<Space>` - Start/Pause/Resume Simulation

## Example Commands

#### Generate - single source

```shell
# generate 21x21 maze using BFS - single source
amazeing create rectangle --maze bfs_21_21.maze --rows 21 --cols 21 --source 10,10 --procedure bfs --unit-shape square
# short
amazeing C R -m bfs_21_21.maze -r 21 -c 21 -s 10,10 -p bfs -U square

# simulate - multi source
amazeing create rectangle --rows 21 --cols 21 --source 10,10 --procedure bfs --verbose --fps 25 --unit-shape hexagonal
# short
amazeing C R -r 21 -c 21 -s 10,10 -p bfs -v -F 25 -U hexagonal
```

#### Generate - multi source

```shell
# generate 21x21 maze using BFS - multi source
amazeing create rectangle --maze bfs_21_21.maze --rows 21 --cols 21 --source 7,7 --source 14,14 --procedure bfs --unit-shape sqaure
# short
amazeing C R -m bfs_21_21.maze -r 21 -c 21 -s 7,7 -s 14,14 -p bfs -U square

# simulate - multi source
amazeing create rectangle --rows 21 --cols 21 --source 7,7 --source 14,14 --procedure bfs --verbose --fps 25 --unit-shape hexagonal
# short
amazeing C R -r 21 -c 21 -s 7,7 -s 14,14 -p bfs -v -F 25 -U hexagonal
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
amazeing solve --maze bfs_21_21.maze --procedure bfs --verbose --fps 25
# short
amazeing S -m bfs_21_21.maze -p bfs -v -F 25
```

## Generate Maze

| Algorithm          | Maze Shape | Unit Shape |                                                                                                  |
|--------------------|------------|------------|--------------------------------------------------------------------------------------------------|
| Dfs                | Rectangle  | Hexagon    | [![](https://img.youtube.com/vi/twafvSeVQOs/0.jpg)](https://www.youtube.com/watch?v=twafvSeVQOs) |
| Dfs (Multi Source) | Rectangle  | Square     | [![](https://img.youtube.com/vi/fL93bHyf6-M/0.jpg)](https://www.youtube.com/watch?v=fL93bHyf6-M) |
| Dfs                | Rectangle  | Square     | [![](https://img.youtube.com/vi/iyxUARc2T2g/0.jpg)](https://www.youtube.com/watch?v=iyxUARc2T2g) |
| Bfs                | Rectangle  | Square     | [![](https://img.youtube.com/vi/st8RLTgAuuE/0.jpg)](https://www.youtube.com/watch?v=st8RLTgAuuE) |
| Dfs                | Triangle   | Hexagon    | [![](https://img.youtube.com/vi/0c4s49G1RAk/0.jpg)](https://www.youtube.com/watch?v=0c4s49G1RAk) |

## Solve Maze

| Algorithm      | Maze Shape | Unit Shape |                                                                                                  |
|----------------|------------|------------|--------------------------------------------------------------------------------------------------|
| A* (Octile)    | Rectangle  | Hexagon    | [![](https://img.youtube.com/vi/MRt7X6JGDuo/0.jpg)](https://www.youtube.com/watch?v=MRt7X6JGDuo) |
| A* (Manhattan) | Rectangle  | Square     | [![](https://img.youtube.com/vi/LkxyikxTX6Y/0.jpg)](https://www.youtube.com/watch?v=LkxyikxTX6Y) |
| DFS            | Rectangle  | Square     | [![](https://img.youtube.com/vi/9F8XRL7lnIU/0.jpg)](https://www.youtube.com/shorts/9F8XRL7lnIU)  |
| BFS            | Rectangle  | Square     | [![](https://img.youtube.com/vi/h8q5vi68fz0/0.jpg)](https://www.youtube.com/shorts/h8q5vi68fz0)  |

## Others

[Color scheme generator](assets/scheme-generator.html)

<iframe src="assets/scheme-generator.html" width="100%" height="920px"></iframe>

## License

This project is licensed under the [GNU AGPL-3.0 License](https://www.gnu.org/licenses/agpl-3.0.html).
See the [LICENSE](./LICENSE) file for more details.
