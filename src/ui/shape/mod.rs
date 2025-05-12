mod create_mesh;
mod hexagon;
mod maze_mesh;
mod octagon;
mod square;
mod triangle;
mod unit_shape_factory_trait;

pub(crate) const MARGIN: f32 = 10.;
pub(crate) const BORDER: f32 = 2.;
pub(crate) const RADIUS: f32 = 15.;

pub(crate) use hexagon::*;
pub(crate) use maze_mesh::*;
pub(crate) use octagon::*;
pub(crate) use square::*;
pub(crate) use triangle::*;
pub(crate) use unit_shape_factory_trait::*;
