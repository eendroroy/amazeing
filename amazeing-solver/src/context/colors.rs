use macroquad::color::{Color, BEIGE, BLACK, DARKGRAY, GOLD, LIGHTGRAY, RED};

#[derive(Debug, Clone)]
pub struct Colors {
    pub(crate) color_bg: Color,
    pub(crate) color_block: Color,
    pub(crate) color_open: Color,
    pub(crate) color_visiting: Color,
    pub(crate) color_path: Color,
    pub(crate) color_traversed: Color,
}

impl Colors {
    pub fn new() -> Self {
        Self {
            color_bg: BLACK,
            color_block: DARKGRAY,
            color_open: LIGHTGRAY,
            color_visiting: RED,
            color_path: GOLD,
            color_traversed: BEIGE,
        }
    }
}
