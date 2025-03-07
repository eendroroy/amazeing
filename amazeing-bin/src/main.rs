use crate::params::parse_params;

pub(crate) mod context;
pub(crate) mod generator;
pub(crate) mod help;
pub(crate) mod matrix;
pub(crate) mod params;
pub(crate) mod solver;

fn main() {
    let mut p = parse_params();

    match &*p.remove(0) {
        "solve" => solver::solve::solve(
            p.remove(0),
            p.remove(0),
            p.remove(0),
            p.remove(0),
            p.remove(0),
            p.remove(0),
        ),
        "generate" => generator::generate::generate(p.remove(0), p.remove(0), p.remove(0)),
        &_ => todo!(),
    }
}
