use crate::command::parse_params;

mod command;
mod context;
mod generator;
mod matrix;
mod solver;

fn main() {
    let mut p = parse_params();

    match &*p.remove(0) {
        "solve" => solver::solve(
            p.remove(0),
            p.remove(0),
            p.remove(0),
            p.remove(0),
            p.remove(0),
            p.remove(0),
        ),
        "generate" => generator::generate(p.remove(0), p.remove(0), p.remove(0), p.remove(0)),
        &_ => todo!(),
    }
}
