use crate::command::parse_params;

mod command;
mod context;
mod display;
mod generator;
mod helper;
mod solver;

fn main() {
    let mut p = parse_params();

    match &*p.remove(0) {
        "simulate" => solver::simulate(
            p.remove(0),
            p.remove(0),
            p.remove(0),
            p.remove(0),
            p.remove(0),
            p.remove(0),
            p.remove(0),
        ),
        "realtime" => solver::realtime(
            p.remove(0),
            p.remove(0),
            p.remove(0),
            p.remove(0),
            p.remove(0),
        ),
        "generate" => generator::generate(
            p.remove(0),
            p.remove(0),
            p.remove(0),
            p.remove(0),
            p.remove(0),
        ),
        "view" => generator::view(p.remove(0), p.remove(0)),
        &_ => todo!(),
    }
}
