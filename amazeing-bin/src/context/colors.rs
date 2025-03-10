use macroquad::color::{Color, BEIGE, BLACK, GOLD, GREEN, LIGHTGRAY, RED};

#[derive(Debug, Clone)]
pub struct Colors {
    pub(crate) color_bg: Color,
    pub(crate) color_block: Color,
    pub(crate) color_open: Color,
    pub(crate) color_visiting: Color,
    pub(crate) color_path: Color,
    pub(crate) color_source: Color,
    pub(crate) color_destination: Color,
    pub(crate) color_traversed: Color,
}

impl Colors {
    pub fn new() -> Self {
        Self {
            color_bg: BLACK,
            color_block: BLACK,
            color_open: LIGHTGRAY,
            color_visiting: RED,
            color_path: GOLD,
            color_source: GREEN,
            color_destination: GREEN,
            color_traversed: BEIGE,
        }
    }
}
