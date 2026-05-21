use crate::cli::{ColorScheme, Colors};
use macroquad::miniquad::window::set_window_size;
use std::path::PathBuf;

/// Resize the application window; adds 30px at the bottom for status/title bar.
pub(super) fn set_screen_size((width, height): (u32, u32)) {
    set_window_size(width, height + 30);
}

/// Build a `Colors` palette from an optional scheme file, or fall back to defaults.
pub(super) fn get_colors(rows: usize, cols: usize, scheme_path: Option<&PathBuf>) -> Colors {
    let steps = gradient_steps(rows, cols);
    match scheme_path {
        Some(path) => Colors::from(ColorScheme::load(path.as_path()), steps),
        None => Colors::new(steps),
    }
}

fn gradient_steps(rows: usize, cols: usize) -> usize {
    ((rows + cols) as f32 * 0.25).clamp(8., 64.) as usize
}
