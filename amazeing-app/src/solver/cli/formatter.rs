use colored::{Color, Colorize};
use crate::solver::matrix::cli_viz::VizType;

pub(crate) fn formatter(d: char, t: VizType) -> String {
    match t {
        VizType::OPEN => String::from(d).color(Color::White).to_string(),
        VizType::BLOCK => String::from(d).color(Color::Red).to_string(),
        VizType::PATH => String::from(d).color(Color::Blue).to_string(),
    }
}
