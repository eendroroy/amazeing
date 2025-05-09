use crate::core::tiled::Rank;
use crate::ui::context::ColorScheme;
use crate::ui::helper::gradient;
use macroquad::prelude::Color;

#[derive(Debug, Clone)]
pub struct ColorContext {
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

impl ColorContext {
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

    pub fn shed_color(&self, rank: &Rank) -> Option<&Color> {
        self.color_visiting_gradient.get((Rank::MAX - rank) as usize)
    }
}
