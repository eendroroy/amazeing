// ── Torch ─────────────────────────────────────────────────────────────────────

/// Radius of the torch-light effect in grid-cell units.
pub(crate) const LIGHT_RADIUS: f32 = 15.0;
/// Minimum brightness for cells far from the torch frontier (0 = black, 1 = full brightness).
pub(crate) const LIGHT_AMBIENT: f32 = 0.5;

// ── Fish-eye ──────────────────────────────────────────────────────────────────

/// Radius of the fish-eye zoom effect in grid-cell units.
pub(crate) const FISHEYE_RADIUS: f32 = 5.0;
/// Zoom strength of the fish-eye zoom effect (fraction of cell size pushed outward at peak).
pub(crate) const ZOOM_STRENGTH: f32 = 0.25;

// ── Glow ──────────────────────────────────────────────────────────────────────

/// Radius of the glow colour-bleed effect in grid-cell units.
pub(crate) const COLOR_SOURCE_RADIUS: f32 = 3.0;
/// Maximum glow blend weight at the peak cell (0 = no tint, 1 = solid source colour).
pub(crate) const COLOR_SOURCE_PEAK: f32 = 0.65;

// ── Shockwave Pulse ───────────────────────────────────────────────────────────

/// Decay radius of the shockwave-pulse distortion effect in grid-cell units.
pub(crate) const SHOCKWAVE_RADIUS: f32 = 10.0;
/// Maximum pixel-space radial displacement at the wave crest.
pub(crate) const SHOCKWAVE_AMPLITUDE: f32 = 5.0;
/// Spatial frequency of the shockwave ripple (higher = more rings per cell).
pub(crate) const SHOCKWAVE_FREQUENCY: f32 = 0.75;
/// Speed at which the shockwave phase advances per second.
pub(crate) const SHOCKWAVE_SPEED: f32 = 5.0;

// ── Gravity Well ──────────────────────────────────────────────────────────────

/// Radius of the gravity-well inward-pull effect in grid-cell units.
pub(crate) const GRAVITY_WELL_RADIUS: f32 = 5.0;
/// Inward pull strength of the gravity-well effect (fraction of cell size pulled inward at peak).
pub(crate) const GRAVITY_WELL_STRENGTH: f32 = 0.20;

// ── Chromatic Wave ────────────────────────────────────────────────────────────

/// Decay radius of the chromatic-wave brightness-ring effect in grid-cell units.
pub(crate) const CHROMATIC_WAVE_RADIUS: f32 = 10.0;
/// Peak brightness modulation amplitude (0 = no wave, 1 = ±100 % luminance swing).
pub(crate) const CHROMATIC_WAVE_AMPLITUDE: f32 = 0.45;
/// Spatial frequency of the chromatic-wave rings (higher = more rings per cell).
pub(crate) const CHROMATIC_WAVE_FREQUENCY: f32 = 0.75;
/// Speed at which the chromatic-wave phase advances per second.
pub(crate) const CHROMATIC_WAVE_SPEED: f32 = 4.0;

