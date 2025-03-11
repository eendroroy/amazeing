use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ColorScheme {
    pub(crate) color_bg: u32,
    pub(crate) color_block: u32,
    pub(crate) color_open: u32,
    pub(crate) color_visiting: u32,
    pub(crate) color_path: u32,
    pub(crate) color_source: u32,
    pub(crate) color_destination: u32,
    pub(crate) color_traversed: u32,
}
