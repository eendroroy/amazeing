use crate::generator;
use macroquad::color::{Color, BLACK, DARKGRAY, LIGHTGRAY};
use std::sync::Mutex;

pub static BG: Color = BLACK;
pub static BLOCK: Color = DARKGRAY;
pub static OPEN: Color = LIGHTGRAY;

pub static PATH: Mutex<String> = Mutex::new(String::new());
pub static ROWS: Mutex<usize> = Mutex::new(10usize);
pub static COLS: Mutex<usize> = Mutex::new(10usize);

pub(crate) fn generate(path: String, rows: String, cols: String) {
    *PATH.lock().unwrap() = path.clone();

    if rows != String::new() {
        *ROWS.lock().unwrap() = usize::from_str_radix(rows.as_str(), 10).unwrap();
    }

    if cols != String::new() {
        *COLS.lock().unwrap() = usize::from_str_radix(cols.as_str(), 10).unwrap();
    }

    generator::manual::manual::main()
}
