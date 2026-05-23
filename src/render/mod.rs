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

/// Radius of the torch-light effect in grid-cell units.
pub(crate) const LIGHT_RADIUS: f32 = 15.0;
/// Minimum brightness for cells far from the light-source frontier (0 = black, 1 = full).
pub(crate) const LIGHT_AMBIENT: f32 = 0.5;

/// Radius of the fish-eye zoom effect in grid-cell units.
pub(crate) const FISHEYE_RADIUS: f32 = 5.0;
/// Zoom strength of the fish-eye zoom effect in grid-cell units.
pub(crate) const ZOOM_STRENGTH: f32 = 0.25;

/// Radius of the colour-source glow effect in grid-cell units.
pub(crate) const COLOR_SOURCE_RADIUS: f32 = 3.0;
/// Maximum colour-source blend weight at the peak cell (0 = no tint, 1 = solid source colour).
pub(crate) const COLOR_SOURCE_PEAK: f32 = 0.65;

/// Decay radius of the shock-wave distortion effect in grid-cell units.
pub(crate) const SHOCKWAVE_RADIUS: f32 = 10.0;
/// Maximum pixel-space radial displacement at the wave crest.
pub(crate) const SHOCKWAVE_AMPLITUDE: f32 = 5.0;
/// Spatial frequency of the shock-wave ripple (higher = more rings per cell).
pub(crate) const SHOCKWAVE_FREQUENCY: f32 = 0.75;
/// Speed at which the shock-wave phase advances per second.
pub(crate) const SHOCKWAVE_SPEED: f32 = 5.0;

