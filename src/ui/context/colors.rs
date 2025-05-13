use crate::core::tiled::Rank;
use crate::ui::helper::gradient;
use macroquad::prelude::Color;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Colors {
    pub(crate) color_bg: Color,
    pub(crate) color_block: Color,
    pub(crate) color_open: Color,
    pub(crate) color_visiting: Color,
    pub(crate) color_visiting_gradient: Vec<Color>,
    pub(crate) color_path: Color,
    pub(crate) color_source: Color,
    pub(crate) color_destination: Color,
    pub(crate) color_traversed: Color,
}

impl Colors {
    pub fn new(steps: usize) -> Self {
        Self {
            color_bg: Color::from_hex(0x00202e),
            color_block: Color::from_hex(0x003f5c),
            color_open: Color::from_hex(0xfff0d4),
            color_visiting: Color::from_hex(0xbc5090),
            color_visiting_gradient: gradient(Color::from_hex(0xff0000), Color::from_hex(0xbc5090), steps),
            color_path: Color::from_hex(0xff6361),
            color_source: Color::from_hex(0xffa600),
            color_destination: Color::from_hex(0xffa600),
            color_traversed: Color::from_hex(0xcfa093),
        }
    }

    pub fn from(scheme: ColorScheme, steps: usize) -> Self {
        Self {
            color_bg: Color::from_hex(scheme.color_bg),
            color_block: Color::from_hex(scheme.color_block),
            color_open: Color::from_hex(scheme.color_open),
            color_visiting: Color::from_hex(scheme.color_visiting),
            color_visiting_gradient: gradient(
                Color::from_hex(scheme.color_visiting_peak),
                Color::from_hex(scheme.color_visiting),
                steps,
            ),
            color_path: Color::from_hex(scheme.color_path),
            color_source: Color::from_hex(scheme.color_source),
            color_destination: Color::from_hex(scheme.color_destination),
            color_traversed: Color::from_hex(scheme.color_traversed),
        }
    }

    pub fn shed_color(&self, rank: &Rank) -> &Color {
        self.color_visiting_gradient
            .get((Rank::MAX - rank) as usize)
            .unwrap_or(&self.color_visiting)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ColorScheme {
    pub(crate) color_bg: u32,
    pub(crate) color_block: u32,
    pub(crate) color_open: u32,
    pub(crate) color_visiting: u32,
    pub(crate) color_path: u32,
    pub(crate) color_visiting_peak: u32,
    pub(crate) color_source: u32,
    pub(crate) color_destination: u32,
    pub(crate) color_traversed: u32,
}

impl ColorScheme {
    pub(crate) fn from(path: &Path) -> Self {
        toml::from_str(&fs::read_to_string(path).unwrap()).unwrap()
    }
}
