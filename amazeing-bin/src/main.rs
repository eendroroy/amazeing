use crate::command::{parse_params, Mode};

mod command;
mod context;
mod display;
mod helper;
mod mode;

fn main() {
    let mode = parse_params();

    match mode {
        Mode::Simulate => mode::simulate(),
        Mode::Realtime => mode::realtime(),
        Mode::Generate => mode::generate(),
        Mode::Visualize => mode::visualize(),
        Mode::Modify => mode::modify(),
        _ => todo!(),
    }
}
