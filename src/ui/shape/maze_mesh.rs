use crate::core::tiled::Node;
use crate::ui::helper::is_point_in_triangle;
use macroquad::models::{Mesh, draw_mesh};
use macroquad::prelude::Color;
use std::ops::{Index, IndexMut};

pub(crate) struct MazeMesh {
    pub(crate) meshes: Vec<Vec<Mesh>>,
}

impl MazeMesh {
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
                if i + 2 < indices.len() && is_point_in_triangle(
                        (x, y),
                        (vertices[indices[i] as usize].position.x, vertices[indices[i] as usize].position.y),
                        (vertices[indices[i + 1] as usize].position.x, vertices[indices[i + 1] as usize].position.y),
                        (vertices[indices[i + 2] as usize].position.x, vertices[indices[i + 2] as usize].position.y),
                    ) {
                    return true;
                }
            }
        }

        false
    }
}

impl Index<Node> for MazeMesh {
    type Output = Mesh;

    fn index(&self, index: Node) -> &Self::Output {
        &self.meshes[index.row][index.col]
    }
}

impl IndexMut<Node> for MazeMesh {
    fn index_mut(&mut self, index: Node) -> &mut Self::Output {
        &mut self.meshes[index.row][index.col]
    }
}
