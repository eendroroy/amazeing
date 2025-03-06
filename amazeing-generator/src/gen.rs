use macroquad::color::{Color, BLACK, DARKGRAY, LIGHTGRAY};
use std::env;
use std::sync::Mutex;

mod manual;
mod matrix;

pub static BG: Color = BLACK;
pub static BLOCK: Color = DARKGRAY;
pub static OPEN: Color = LIGHTGRAY;

pub static PATH: Mutex<String> = Mutex::new(String::new());
pub static ROWS: Mutex<usize> = Mutex::new(10usize);
pub static COLS: Mutex<usize> = Mutex::new(10usize);

fn main() {
    let mut args = env::args().skip(1);

    let mut path = String::from("");
    let mut rows = String::from("");
    let mut cols = String::from("");

    while let Some(arg) = args.next() {
        match &arg[..] {
            "--path" => path = args.next().unwrap(),
            "--rows" => rows = args.next().unwrap(),
            "--cols" => cols = args.next().unwrap(),
            _ => {
                if arg.starts_with('-') {
                    println!("Unknown argument {}", arg);
                } else {
                    println!("Unknown positional argument {}", arg);
                }
            }
        }
    }


    *PATH.lock().unwrap() = path.clone();

    if rows != String::new() {
        *ROWS.lock().unwrap() = usize::from_str_radix(rows.as_str(), 10).unwrap();
    }

    if cols != String::new() {
        *COLS.lock().unwrap() = usize::from_str_radix(cols.as_str(), 10).unwrap();
    }


    manual::manual::main()
}
