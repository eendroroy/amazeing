use crate::core::tiled::{Maze, Node, UnitShape, BLOCK, OPEN, VOID};
use crate::ui::context::Colors;
use crate::ui::helper::is_point_in_triangle;
use crate::ui::shape::{
    HexagonUnitShapeFactory, OctagonUnitShapeFactory, SquareUnitShapeFactory, TriangleUnitShapeFactory,
    UnitShapeFactory,
};
use macroquad::models::{draw_mesh, Mesh};
use macroquad::prelude::Color;
use std::ops::{Index, IndexMut};

pub(crate) struct MazeScene {
    pub(crate) meshes: Vec<Vec<Mesh>>,
    pub(crate) maze: Maze,
    pub(crate) colors: Colors,
    pub(crate) dimension: (u32, u32),
}

impl MazeScene {
    pub(crate) fn new(maze: &Maze, unit_shape: UnitShape, zoom: f32, colors: &Colors) -> Self {
        let shape_factory: Box<dyn UnitShapeFactory> = match unit_shape {
            UnitShape::Triangle => Box::new(TriangleUnitShapeFactory::new(zoom)),
            UnitShape::Square => Box::new(SquareUnitShapeFactory::new(zoom)),
            UnitShape::Hexagon => Box::new(HexagonUnitShapeFactory::new(zoom)),
            UnitShape::Octagon => Box::new(OctagonUnitShapeFactory::new(zoom)),
        };

        let meshes = maze
            .data
            .iter()
            .enumerate()
            .map(|(row, row_data)| {
                row_data
                    .iter()
                    .enumerate()
                    .map(|(col, &cell)| {
                        let color = match cell {
                            OPEN => colors.color_open,
                            BLOCK => colors.color_block,
                            VOID => colors.color_bg,
                            _ => colors.color_bg,
                        };
                        shape_factory.shape(row, col, color)
                    })
                    .collect::<Vec<Mesh>>()
            })
            .collect::<Vec<Vec<Mesh>>>();

        Self {
            meshes,
            maze: maze.clone(),
            colors: colors.clone(),
            dimension: shape_factory.dimension(maze.rows(), maze.cols()),
        }
    }
    pub(crate) fn draw(&self) {
        self.meshes.iter().for_each(|row| row.iter().for_each(draw_mesh));
    }

    pub(crate) fn clicked_on(&self, (x, y): (f32, f32)) -> Option<Node> {
        for (row_idx, row) in self.meshes.iter().enumerate() {
            for (col_idx, mesh) in row.iter().enumerate() {
                if self.is_point_in_mesh(mesh, x, y) {
                    return Node::new(self.meshes.len() - 1, row.len() - 1).at(row_idx, col_idx);
                }
            }
        }

        // Default return if no mesh contains the point
        None
    }

    pub(crate) fn update_color(&mut self, node: Node, color: Color) {
        self[node]
            .vertices
            .iter_mut()
            .for_each(|vertex| vertex.color = color.into())
    }

    fn is_point_in_mesh(&self, mesh: &Mesh, x: f32, y: f32) -> bool {
        let vertices = &mesh.vertices;
        let indices = &mesh.indices;

        if indices.len() >= 3 {
            for i in (0..indices.len()).step_by(3) {
                if i + 2 < indices.len()
                    && is_point_in_triangle(
                        (x, y),
                        (vertices[indices[i] as usize].position.x, vertices[indices[i] as usize].position.y),
                        (vertices[indices[i + 1] as usize].position.x, vertices[indices[i + 1] as usize].position.y),
                        (vertices[indices[i + 2] as usize].position.x, vertices[indices[i + 2] as usize].position.y),
                    )
                {
                    return true;
                }
            }
        }

        false
    }
}

impl Index<Node> for MazeScene {
    type Output = Mesh;

    fn index(&self, index: Node) -> &Self::Output {
        &self.meshes[index.row][index.col]
    }
}

impl IndexMut<Node> for MazeScene {
    fn index_mut(&mut self, index: Node) -> &mut Self::Output {
        &mut self.meshes[index.row][index.col]
    }
}
