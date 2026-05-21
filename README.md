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
amzeing --help
```

### Key/Mouse events

- Common (all interactive modes)
    - `Control`+`I` - Save current screen as PNG in current directory
    - `Q` - Quit current view

- Create (`create`)
    - `LeftClick` - Add/Remove Source (before generation starts)
    - `Shift`+`LeftClick` - Set Destination (before generation starts; required for `a-star`/`bidirectional-a-start`)
    - `G`/`<Space>` - Generate maze / Start simulation
    - Additional actions in simulation mode (`--verbose`/`-v`):
        - `<Space>` - Pause/Resume simulation
        - `R` - Restart simulation and reset selection
    - Additional action in non-simulation mode:
        - `Control`+`S` - Save current maze to `--maze` path (if provided)

- View (`view`) - Update mode (`--update`/`-u`):
    - `LeftClick` - Open path
    - `Shift`+`LeftClick` - Block path
    - `Control`+`S` - Save current maze to the same file

- Solve (`solve`)
    - `LeftClick` - Select Source
    - `Shift`+`LeftClick` - Select Destination
    - Additional action in simulation mode (`--verbose`/`-v`):
        - `S`/`<Space>` - Start simulation
        - `<Space>` - Pause/Resume simulation
        - `R` - Restart simulation and reset selection

## Generate Maze

| Algorithm          | Maze Shape | Unit Shape |                                                                                                  |
|--------------------|------------|------------|--------------------------------------------------------------------------------------------------|
| Dfs                | Rectangle  | Hexagon    | [![](https://img.youtube.com/vi/twafvSeVQOs/0.jpg)](https://www.youtube.com/watch?v=twafvSeVQOs) |
| Dfs (Multi Source) | Rectangle  | Square     | [![](https://img.youtube.com/vi/fL93bHyf6-M/0.jpg)](https://www.youtube.com/watch?v=fL93bHyf6-M) |
| Dfs                | Rectangle  | Square     | [![](https://img.youtube.com/vi/iyxUARc2T2g/0.jpg)](https://www.youtube.com/watch?v=iyxUARc2T2g) |
| Bfs                | Rectangle  | Square     | [![](https://img.youtube.com/vi/st8RLTgAuuE/0.jpg)](https://www.youtube.com/watch?v=st8RLTgAuuE) |
| Dfs                | Triangle   | Hexagon    | [![](https://img.youtube.com/vi/0c4s49G1RAk/0.jpg)](https://www.youtube.com/watch?v=0c4s49G1RAk) |
| A* (Manhattan)     | Circle     | Hexagon    | [![](https://img.youtube.com/vi/CZanPMEyYZE/0.jpg)](https://www.youtube.com/shorts/CZanPMEyYZE)  |

## Solve Maze

| Algorithm      | Maze Shape | Unit Shape |                                                                                                  |
|----------------|------------|------------|--------------------------------------------------------------------------------------------------|
| A* (Octile)    | Rectangle  | Hexagon    | [![](https://img.youtube.com/vi/MRt7X6JGDuo/0.jpg)](https://www.youtube.com/watch?v=MRt7X6JGDuo) |
| A* (Manhattan) | Rectangle  | Square     | [![](https://img.youtube.com/vi/LkxyikxTX6Y/0.jpg)](https://www.youtube.com/watch?v=LkxyikxTX6Y) |
| DFS            | Rectangle  | Square     | [![](https://img.youtube.com/vi/9F8XRL7lnIU/0.jpg)](https://www.youtube.com/shorts/9F8XRL7lnIU)  |
| BFS            | Rectangle  | Square     | [![](https://img.youtube.com/vi/h8q5vi68fz0/0.jpg)](https://www.youtube.com/shorts/h8q5vi68fz0)  |

## Others

[Color scheme generator](assets/scheme-generator.html)

## License

This project is licensed under the [GNU AGPL-3.0 License](https://www.gnu.org/licenses/agpl-3.0.html).
See the [LICENSE](./LICENSE) file for more details.
