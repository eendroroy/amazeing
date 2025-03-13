mod color_context;
mod color_scheme;
mod draw_context;
mod generation_context;
mod realtime_context;
mod simulation_context;
mod visualize_context;

pub(crate) use color_context::ColorContext;
pub(crate) use color_context::COL_CTX;
pub(crate) use color_scheme::ColorScheme;
pub(crate) use draw_context::DRAW_CTX;
pub(crate) use generation_context::GEN_CTX;
pub(crate) use realtime_context::REL_CTX;
pub(crate) use simulation_context::SIM_CTX;
pub(crate) use visualize_context::VIS_CTX;
