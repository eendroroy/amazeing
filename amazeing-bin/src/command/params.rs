use crate::command::args::{Param, NAN_PARAM};
use crate::command::help;
use crate::command::mode::Mode;
use std::env;
use std::env::Args;
use std::iter::Skip;

pub(crate) fn parse_params() -> Mode {
    let mut args = env::args().skip(1);

    let mut mode = Mode::None { name: NAN_PARAM };

    while let Some(arg) = args.next() {
        match &arg[..] {
            val if val == Param::Help.value() => help::help(),
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
                mode = Mode::None { name: arg.clone() };
                break;
            }
        }
    }

    match mode {
        Mode::Generate => parse_generate_params(args),
        Mode::Simulate => parse_solve_params(args),
        Mode::Realtime => parse_realtime_params(args),
        Mode::Modify | Mode::Visualize => parse_view_edit_params(args),
        Mode::None { name: arg } => panic!("Invalid usage: [mode = {}]", arg),
    };

    mode
}

fn parse_solve_params(mut args: Skip<Args>) {
    while let Some(arg) = args.next() {
        match &arg[..] {
            val if val == Param::Maze.value() => Param::Maze.set(args.next().unwrap()),
            val if val == Param::Source.value() => Param::Source.set(args.next().unwrap()),
            val if val == Param::Target.value() => Param::Target.set(args.next().unwrap()),
            val if val == Param::Proc.value() => Param::Proc.set(args.next().unwrap()),
            val if val == Param::Heu.value() => Param::Heu.set(args.next().unwrap()),
            val if val == Param::Fps.value() => Param::Fps.set(args.next().unwrap()),
            val if val == Param::Display.value() => Param::Display.set(args.next().unwrap()),
            val if val == Param::Colors.value() => Param::Colors.set(args.next().unwrap()),
            _ => println!("Unknown argument {}", arg),
        }
    }
}

fn parse_realtime_params(mut args: Skip<Args>) {
    while let Some(arg) = args.next() {
        match &arg[..] {
            val if val == Param::Maze.value() => Param::Maze.set(args.next().unwrap()),
            val if val == Param::Proc.value() => Param::Proc.set(args.next().unwrap()),
            val if val == Param::Heu.value() => Param::Heu.set(args.next().unwrap()),
            val if val == Param::Display.value() => Param::Display.set(args.next().unwrap()),
            val if val == Param::Colors.value() => Param::Colors.set(args.next().unwrap()),
            _ => println!("Unknown argument {}", arg),
        }
    }
}

fn parse_generate_params(mut args: Skip<Args>) {
    while let Some(arg) = args.next() {
        match &arg[..] {
            val if val == Param::Maze.value() => Param::Maze.set(args.next().unwrap()),
            val if val == Param::Rows.value() => Param::Rows.set(args.next().unwrap()),
            val if val == Param::Cols.value() => Param::Cols.set(args.next().unwrap()),
            val if val == Param::Proc.value() => Param::Proc.set(args.next().unwrap()),
            val if val == Param::Display.value() => Param::Display.set(args.next().unwrap()),
            val if val == Param::Colors.value() => Param::Colors.set(args.next().unwrap()),
            _ => println!("Unknown argument {}", arg),
        }
    }
}

fn parse_view_edit_params(mut args: Skip<Args>) {
    while let Some(arg) = args.next() {
        match &arg[..] {
            val if val == Param::Maze.value() => Param::Maze.set(args.next().unwrap()),
            val if val == Param::Display.value() => Param::Display.set(args.next().unwrap()),
            val if val == Param::Colors.value() => Param::Colors.set(args.next().unwrap()),
            _ => println!("Unknown argument {}", arg),
        }
    }
}
