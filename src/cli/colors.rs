use crate::maze::Rank;
use crate::render::helper::gradient;
use macroquad::color::Color;
use serde::Deserialize;
use std::fs;
use std::path::Path;

/// A resolved color palette used by the UI when rendering a maze.
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
    pub(crate) color_perimeter: Color,
}

impl Colors {
    /// Default dark-ocean palette with `steps` gradient shades for the visiting animation.
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
            color_perimeter: Color::from_hex(0xc9c982),
        }
    }

    /// Build a palette from a user-supplied `ColorScheme` TOML with `steps` gradient shades.
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
            color_perimeter: Color::from_hex(scheme.color_perimeter),
        }
    }

    /// Returns the gradient color corresponding to the given rank value.
    pub fn shed_color(&self, rank: &Rank) -> &Color {
        self.color_visiting_gradient
            .get((Rank::MAX - rank) as usize)
            .unwrap_or(&self.color_visiting)
    }
}

/// Deserialised representation of an `assets/scheme/*.toml` color-scheme file.
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
    pub(crate) color_perimeter: u32,
}

impl ColorScheme {
    pub(crate) fn from(path: &Path) -> Self {
        toml::from_str(&fs::read_to_string(path).unwrap()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn colors_new_and_shed_color_work() {
        let colors = Colors::new(4);
        assert_eq!(colors.color_visiting_gradient.len(), 4);

        let c = colors.shed_color(&Rank::MAX);
        assert_eq!(c.r, colors.color_visiting_gradient[0].r);

        let fallback = colors.shed_color(&(Rank::MAX - 9999));
        assert_eq!(fallback.r, colors.color_visiting.r);
    }

    #[test]
    fn color_scheme_from_file_and_palette_from_scheme_work() {
        let path = std::env::temp_dir().join(format!(
            "amazeing_scheme_{}.toml",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));

        let mut f = std::fs::File::create(&path).unwrap();
        writeln!(f, "color_bg=0x000000").unwrap();
        writeln!(f, "color_block=0x111111").unwrap();
        writeln!(f, "color_open=0x222222").unwrap();
        writeln!(f, "color_visiting=0x333333").unwrap();
        writeln!(f, "color_path=0x444444").unwrap();
        writeln!(f, "color_visiting_peak=0x555555").unwrap();
        writeln!(f, "color_source=0x666666").unwrap();
        writeln!(f, "color_destination=0x777777").unwrap();
        writeln!(f, "color_traversed=0x888888").unwrap();
        writeln!(f, "color_perimeter=0x999999").unwrap();

        let scheme = ColorScheme::from(&path);
        let colors = Colors::from(scheme, 3);
        assert_eq!(colors.color_visiting_gradient.len(), 3);

        let _ = std::fs::remove_file(path);
    }
}
