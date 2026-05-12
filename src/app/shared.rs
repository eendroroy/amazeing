use crate::cli::{ColorScheme, Colors};
use macroquad::miniquad::window::set_window_size;
use std::path::PathBuf;

pub(super) fn set_screen_size((width, height): (u32, u32)) {
    set_window_size(width, height + 30);
}

pub(super) fn get_colors(rows: usize, cols: usize, scheme_path: Option<&PathBuf>) -> Colors {
    let gradient_steps = gradient_steps(rows, cols);
    if let Some(path) = scheme_path {
        Colors::from(ColorScheme::from(path.as_path()), gradient_steps)
    } else {
        Colors::new(gradient_steps)
    }
}

fn gradient_steps(rows: usize, cols: usize) -> usize {
    ((rows + cols) as f32 * 0.25).clamp(8., 64.) as usize
}
