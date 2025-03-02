use amazeing::solver::matrix::Maze;
use macroquad::color::{Color, BLACK, DARKGRAY, LIGHTGRAY};
use macroquad::prelude::{clear_background, draw_rectangle};

pub(crate) fn draw(
    maze: &Maze,
    margin: f32,
    padding: f32,
    cell_width: f32,
    cell_height: f32,
    path: Vec<(usize, usize)>,
    path_color: Color,
) {
    clear_background(BLACK);

    for r in 0..maze.rows() {
        for c in 0..maze.cols() {
            let color: Color = if path.contains(&(r, c)) {
                path_color
            } else if maze[(r, c)] > 0 {
                LIGHTGRAY
            } else {
                DARKGRAY
            };

            draw_rectangle(
                margin + c as f32 * (cell_width + padding),
                margin + r as f32 * (cell_height + padding),
                cell_width,
                cell_height,
                color,
            );
        }
    }
}
