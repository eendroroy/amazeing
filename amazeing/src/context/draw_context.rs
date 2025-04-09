use crate::command::ArgDisplayDensity;
use amazeing::matrix::{Node, Shape};
use std::f32::consts::PI;

#[derive(Debug, Clone)]
pub struct DrawContext {
    pub(crate) maze_rows: usize,
    pub(crate) maze_cols: usize,
    pub(crate) margin: f32,
    pub(crate) padding: f32,
    pub(crate) size: f32,
    pub(crate) shape: Shape,
    pub(crate) height: f32,
    pub(crate) width: f32,
}

impl DrawContext {
    pub fn new() -> Self {
        Self {
            maze_rows: 0,
            maze_cols: 0,
            margin: 15.,
            padding: 2.,
            size: 15.,
            shape: Shape::Square,
            height: 0.,
            width: 0.,
        }
    }

    pub fn size(&mut self, data: (f32, f32, f32)) {
        (self.margin, self.padding, self.size) = data;
        if self.shape == Shape::Hexagon {
            self.height = ((PI / 6.).sin() + 1.0) * self.size;
            self.width = (PI / 6.).cos() * self.size * 2.0;
        }
    }

    pub fn density(&mut self, density: ArgDisplayDensity) {
        match density {
            ArgDisplayDensity::Connected => self.padding = 0.,
            ArgDisplayDensity::Dense => self.padding = (self.padding / 1.5).ceil(),
            ArgDisplayDensity::Standard => {}
            ArgDisplayDensity::Cozy => self.padding = (self.padding * 1.5).ceil(),
            ArgDisplayDensity::Ample => self.padding = (self.padding * 2.).ceil(),
        }
    }

    pub fn screen_size(&self) -> (u32, u32) {
        match self.shape {
            Shape::Square => (
                (self.margin * 2. + self.maze_cols as f32 * (self.size + self.padding)) as u32,
                (self.margin * 2. + self.maze_rows as f32 * (self.size + self.padding)) as u32,
            ),
            Shape::Hexagon => {
                let width =
                    (self.margin * 2. + self.maze_cols as f32 * (self.width + self.padding)) as u32 + self.size as u32;
                let height = (self.margin * 2. + self.maze_rows as f32 * (self.height + self.padding)) as u32;
                (width, height)
            }
        }
    }

    pub fn x(&self, node: Node) -> f32 {
        match self.shape {
            Shape::Square => self.margin + node.1 as f32 * (self.size + self.padding),
            Shape::Hexagon => {
                (self.margin + self.size + (node.1 as f32 * self.width) + self.padding * node.1 as f32) + self.s(node.0)
            }
        }
    }

    pub fn y(&self, node: Node) -> f32 {
        match self.shape {
            Shape::Square => self.margin + node.0 as f32 * (self.size + self.padding),
            Shape::Hexagon => self.margin + self.size + (node.0 as f32 * self.height) + self.padding * node.0 as f32,
        }
    }

    pub fn s(&self, m: usize) -> f32 {
        if m % 2 == 1 { self.width / 2.0 } else { 0. }
    }
}
