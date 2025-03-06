use crate::params::parse_params;

pub(crate) mod context;
pub(crate) mod generate;
pub(crate) mod generator;
pub(crate) mod help;
pub(crate) mod matrix;
pub(crate) mod params;
pub(crate) mod solve;
pub(crate) mod solver;

fn main() {
    let mut p = parse_params();

    match &*p.remove(0) {
        "solve" => {
            solve::solve(
                p[0].clone(),
                p[1].clone(),
                p[2].clone(),
                p[3].clone(),
                p[4].clone(),
                p[5].clone(),
            );
        }
        "generate" => generate::generate(p[0].clone(), p[1].clone(), p[2].clone()),
        &_ => todo!(),
    }
}
