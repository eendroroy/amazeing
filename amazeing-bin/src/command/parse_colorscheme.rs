use crate::command::color_scheme::ColorScheme;
use crate::context::Colors;
use std::fs;

pub(crate) fn parse_colorscheme(path: &str) -> Colors {
    let scheme: ColorScheme = toml::from_str(&*fs::read_to_string(path).unwrap()).unwrap();
    Colors::from(scheme)
}
