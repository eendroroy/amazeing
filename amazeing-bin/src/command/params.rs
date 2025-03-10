use crate::command::help;
use crate::command::mode::Mode;
use crate::command::parse_node::parse_node;
use crate::context::CONTEXT;
use crate::helper::loader::loader_maze_from_file;
use std::env;
use std::env::Args;
use std::iter::Skip;

pub(crate) fn parse_params() -> Mode {
    let mut args = env::args().skip(1);

    let mut mode = Mode::None;

    while let Some(arg) = args.next() {
        match &arg[..] {
            "--help" => help::help(),
            "--generate" => {
                mode = Mode::Generate;
                break;
            }
            "--view" => {
                mode = Mode::Visualize;
                break;
            }
            "--modify" => {
                mode = Mode::Modify;
                break;
            }
            "--simulate" => {
                mode = Mode::Simulate;
                break;
            }
            "--realtime" => {
                mode = Mode::Realtime;
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

    match mode {
        Mode::Generate => parse_generate_params(args),
        Mode::Simulate => parse_solve_params(args),
        Mode::Realtime => parse_realtime_params(args),
        Mode::Modify | Mode::Visualize => parse_view_edit_params(args),
        Mode::None => panic!("Invalid usage [mode = None]"),
    };

    mode
}

fn parse_solve_params(mut args: Skip<Args>) {
    let mut proc = String::from("");
    let mut heu = String::from("");
    let mut maze_file_path = String::from("");
    let mut from = String::from("");
    let mut to = String::from("");
    let mut fps = String::from("");
    let mut display_size = String::from("");

    while let Some(arg) = args.next() {
        match &arg[..] {
            "--proc" => proc = args.next().unwrap(),
            "--heu" => heu = args.next().unwrap(),
            "--maze" => maze_file_path = args.next().unwrap(),
            "--from" => from = args.next().unwrap(),
            "--to" => to = args.next().unwrap(),
            "--fps" => fps = args.next().unwrap(),
            "--display" => display_size = args.next().unwrap(),
            _ => println!("Unknown argument {}", arg),
        }
    }

    CONTEXT.write().unwrap().maze_file_path = maze_file_path.clone();
    CONTEXT.write().unwrap().maze = loader_maze_from_file(&maze_file_path).clone();

    CONTEXT.write().unwrap().proc = proc;
    CONTEXT.write().unwrap().heu = heu;
    CONTEXT.write().unwrap().source = parse_node(&from);
    CONTEXT.write().unwrap().destination = parse_node(&to);
    CONTEXT.write().unwrap().fps = u8::from_str_radix(&fps, 10).unwrap();
    CONTEXT.write().unwrap().display_size = display_size;
}

fn parse_realtime_params(mut args: Skip<Args>) {
    let mut proc = String::from("");
    let mut heu = String::from("");
    let mut maze_file_path = String::from("");
    let mut display_size = String::from("");

    while let Some(arg) = args.next() {
        match &arg[..] {
            "--proc" => proc = args.next().unwrap(),
            "--heu" => heu = args.next().unwrap(),
            "--maze" => maze_file_path = args.next().unwrap(),
            "--display" => display_size = args.next().unwrap(),
            _ => println!("Unknown argument {}", arg),
        }
    };

    CONTEXT.write().unwrap().maze_file_path = maze_file_path.clone();
    CONTEXT.write().unwrap().maze = loader_maze_from_file(&maze_file_path).clone();

    CONTEXT.write().unwrap().proc = proc;
    CONTEXT.write().unwrap().heu = heu;
    CONTEXT.write().unwrap().display_size = display_size;
}

fn parse_generate_params(mut args: Skip<Args>) {
    let mut maze_file_path = String::from("");
    let mut rows = String::from("");
    let mut cols = String::from("");
    let mut proc = String::from("");
    let mut display_size = String::from("");

    while let Some(arg) = args.next() {
        match &arg[..] {
            "--maze" => maze_file_path = args.next().unwrap(),
            "--rows" => rows = args.next().unwrap(),
            "--cols" => cols = args.next().unwrap(),
            "--proc" => proc = args.next().unwrap(),
            "--display" => display_size = args.next().unwrap(),
            _ => println!("Unknown argument {}", arg),
        }
    }

    CONTEXT.write().unwrap().maze_file_path = maze_file_path;
    CONTEXT.write().unwrap().proc = proc;
    CONTEXT.write().unwrap().rows = usize::from_str_radix(&rows, 10).unwrap();
    CONTEXT.write().unwrap().cols = usize::from_str_radix(&cols, 10).unwrap();
    CONTEXT.write().unwrap().display_size = display_size;
}

fn parse_view_edit_params(mut args: Skip<Args>) {
    let mut maze_file_path = String::from("");
    let mut display_size = String::from("");

    while let Some(arg) = args.next() {
        match &arg[..] {
            "--maze" => maze_file_path = args.next().unwrap(),
            "--display" => display_size = args.next().unwrap(),
            _ => println!("Unknown argument {}", arg),
        }
    }

    CONTEXT.write().unwrap().maze_file_path = maze_file_path.clone();
    CONTEXT.write().unwrap().maze = loader_maze_from_file(&maze_file_path).clone();
    CONTEXT.write().unwrap().display_size = display_size;
}
