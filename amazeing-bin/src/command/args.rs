use crate::command::parse_colorscheme::parse_colorscheme;
use crate::command::parse_node::parse_node;
use crate::command::{get_heuristic, get_proc, help};
use crate::context::CONTEXT;
use crate::helper::loader::loader_maze_from_file;
use std::process::exit;

pub const NAN_PARAM: String = String::new();

pub enum Param {
    Help,
    Maze,
    Source,
    Target,
    Proc,
    Heu,
    Rows,
    Cols,
    Fps,
    Display,
    Colors,
}

impl Param {
    fn show_help_then_exit(&self) {
        println!("{} is not given", self.value());
        help();
        exit(2);
    }

    pub fn value(&self) -> String {
        String::from(match self {
            Param::Help => "--help",
            Param::Maze => "--maze",
            Param::Source => "--source",
            Param::Target => "--target",
            Param::Proc => "--proc",
            Param::Heu => "--heu",
            Param::Rows => "--rows",
            Param::Cols => "--cols",
            Param::Fps => "--fps",
            Param::Display => "--display",
            Param::Colors => "--colors",
        })
    }

    pub fn set(&self, value: String) {
        match self {
            Param::Help => {}
            Param::Maze => {
                if is_given(&value) {
                    CONTEXT.write().unwrap().maze_file_path = value.clone();
                    CONTEXT.write().unwrap().maze = loader_maze_from_file(&value).clone();
                } else {
                    self.show_help_then_exit();
                }
            }
            Param::Source => {
                if is_given(&value) {
                    CONTEXT.write().unwrap().source = parse_node(&value);
                } else {
                    self.show_help_then_exit();
                }
            }
            Param::Target => {
                if is_given(&value) {
                    CONTEXT.write().unwrap().destination = parse_node(&value);
                } else {
                    self.show_help_then_exit();
                }
            }
            Param::Proc => {
                if is_given(&value) {
                    CONTEXT.write().unwrap().proc = get_proc(&value);
                } else {
                    self.show_help_then_exit();
                }
            }
            Param::Heu => {
                if is_given(&value) {
                    CONTEXT.write().unwrap().heuristic = get_heuristic(&*value);
                } else {
                    self.show_help_then_exit();
                }
            }
            Param::Rows => {
                if is_given(&value) {
                    CONTEXT.write().unwrap().rows = usize::from_str_radix(&value, 10).unwrap();
                } else {
                    self.show_help_then_exit();
                }
            }
            Param::Cols => {
                if is_given(&value) {
                    CONTEXT.write().unwrap().cols = usize::from_str_radix(&value, 10).unwrap();
                } else {
                    self.show_help_then_exit();
                }
            }
            Param::Fps => {
                if is_given(&value) {
                    CONTEXT.write().unwrap().fps = u8::from_str_radix(&value, 10).unwrap();
                } else {
                    self.show_help_then_exit();
                }
            }
            Param::Display => {
                if is_given(&value) {
                    CONTEXT.write().unwrap().display_size = value.clone();
                } else {
                    self.show_help_then_exit();
                }
            }
            Param::Colors => {
                if is_given(&value) {
                    CONTEXT.write().unwrap().colors = parse_colorscheme(&*value);
                } else {
                    self.show_help_then_exit();
                }
            }
        }
    }
}

fn is_given(param: &String) -> bool {
    *param != NAN_PARAM
}
