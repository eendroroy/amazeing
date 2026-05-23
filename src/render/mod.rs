pub(crate) mod display_loop;
pub(crate) mod effects;
pub(crate) mod helper;
pub(crate) mod scene;
pub(crate) mod unit;

// Re-export all effect constants so existing `crate::render::*` import paths
// continue to work without modification.
pub(crate) use effects::*;

/// Margin around the drawn grid (pixels at base zoom).
pub(crate) const MARGIN: f32 = 15.;
/// Gap between adjacent cells (pixels at base zoom).
pub(crate) const BORDER: f32 = 5.;
/// Circumscribed-circle radius of one cell (pixels at base zoom).
pub(crate) const RADIUS: f32 = 20.;
