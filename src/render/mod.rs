pub(crate) mod display_loop;
pub(crate) mod helper;
pub(crate) mod scene;
pub(crate) mod unit;

/// Margin around the drawn grid (pixels at base zoom).
pub(crate) const MARGIN: f32 = 15.;
/// Gap between adjacent cells (pixels at base zoom).
pub(crate) const BORDER: f32 = 5.;
/// Circumscribed-circle radius of one cell (pixels at base zoom).
pub(crate) const RADIUS: f32 = 20.;
