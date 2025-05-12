mod create_mesh;
mod maze_mesh;
mod unit_hexagon;
mod unit_octagon;
mod unit_shape_factory_trait;
mod unit_square;
mod unit_triangle;

pub(crate) use maze_mesh::*;
pub(crate) use unit_hexagon::*;
pub(crate) use unit_octagon::*;
pub(crate) use unit_shape_factory_trait::*;
pub(crate) use unit_square::*;
pub(crate) use unit_triangle::*;

pub(crate) const MARGIN: f32 = 10.;
pub(crate) const BORDER: f32 = 2.;
pub(crate) const RADIUS: f32 = 15.;
