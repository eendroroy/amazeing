use crate::command::help;
use std::env::Args;
use std::iter::Skip;
use std::{env, vec};

pub(crate) fn parse_params() -> Vec<String> {
    let mut args = env::args().skip(1);

    let mut mode = String::from("");

    while let Some(arg) = args.next() {
        match &arg[..] {
            "-h" | "--help" => help::help(),
            "-s" | "--solve" => {
                mode = String::from("solve");
                break;
            }
            "-g" | "--generate" => {
                mode = String::from("generate");
                break;
            }
            _ => {
                if arg.starts_with('-') {
                    println!("Unknown argument {}", arg);
                } else {
                    println!("Unknown positional argument {}", arg);
                }
            }
        }
    }

    let mut mode_args = match &mode[..] {
        "solve" => parse_solve_params(args),
        "generate" => parse_gen_params(args),
        _ => vec![],
    };

    mode_args.insert(0, mode);
    mode_args
}

fn parse_solve_params(mut args: Skip<Args>) -> Vec<String> {
    let mut algorithm = String::from("");
    let mut heu = String::from("");
    let mut maze_file_path = String::from("");
    let mut from = String::from("");
    let mut to = String::from("");
    let mut fps = String::from("");

    while let Some(arg) = args.next() {
        match &arg[..] {
            "--algorithm" => algorithm = args.next().unwrap(),
            "--heu" => heu = args.next().unwrap(),
            "--maze" => maze_file_path = args.next().unwrap(),
            "--from" => from = args.next().unwrap(),
            "--to" => to = args.next().unwrap(),
            "--fps" => fps = args.next().unwrap(),
            _ => {
                if arg.starts_with('-') {
                    println!("Unknown argument {}", arg);
                } else {
                    println!("Unknown positional argument {}", arg);
                }
            }
        }
    }

    vec![String::from(algorithm), heu, maze_file_path, from, to, fps]
}

fn parse_gen_params(mut args: Skip<Args>) -> Vec<String> {
    let mut maze_file_path = String::from("");
    let mut rows = String::from("");
    let mut cols = String::from("");
    let mut proc = String::from("");

    while let Some(arg) = args.next() {
        match &arg[..] {
            "--maze" => maze_file_path = args.next().unwrap(),
            "--rows" => rows = args.next().unwrap(),
            "--cols" => cols = args.next().unwrap(),
            "--proc" => proc = args.next().unwrap(),
            _ => {
                if arg.starts_with('-') {
                    println!("Unknown argument {}", arg);
                } else {
                    println!("Unknown positional argument {}", arg);
                }
            }
        }
    }

    vec![maze_file_path, rows, cols, proc]
}
