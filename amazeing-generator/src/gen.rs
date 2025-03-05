use macroquad::color::{Color, BLACK, DARKGRAY, LIGHTGRAY};
use std::path::Path;
use std::sync::Mutex;
use std::{env, fs};

mod manual;
mod matrix;

pub static BG: Color = BLACK;
pub static BLOCK: Color = DARKGRAY;
pub static OPEN: Color = LIGHTGRAY;

pub static PATH: Mutex<String> = Mutex::new(String::new());

fn main() {
    let mut args = env::args().skip(1);

    let mut path = String::from("");
    let mut size = String::from("");

    while let Some(arg) = args.next() {
        match &arg[..] {
            "--path" => path = args.next().unwrap(),
            "--size" => size = args.next().unwrap(),
            _ => {
                if arg.starts_with('-') {
                    println!("Unknown argument {}", arg);
                } else {
                    println!("Unknown positional argument {}", arg);
                }
            }
        }
    }

    if fs::exists(Path::new(&path)).unwrap() {
        panic!("Maze file {} already exists", path)
    } else {
        *PATH.lock().unwrap() = path.clone();
    }


    manual::manual::main()
}
