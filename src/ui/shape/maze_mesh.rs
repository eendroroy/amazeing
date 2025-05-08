use crate::core::tiled::Node;
use crate::ui::shape::ShapeFactory;
use macroquad::models::{Mesh, draw_mesh};
use std::ops::{Index, IndexMut};

pub(crate) struct MazeMesh {
    pub(crate) meshes: Vec<Vec<Mesh>>,
    pub(crate) shape_factory: Box<dyn ShapeFactory>,
}

impl MazeMesh {
    pub(crate) fn draw(&self) {
        self.meshes
            .iter()
            .for_each(|row| row.iter().for_each(|cell| draw_mesh(&cell)));
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

    fn is_point_in_mesh(&self, mesh: &Mesh, x: f32, y: f32) -> bool {
        // Use the shape factory to delegate the point-in-shape test based on shape type
        let point = (x, y);

        // Extract vertices from the mesh
        let vertices = &mesh.vertices;
        let indices = &mesh.indices;

        // For triangulated meshes, we need to check if the point is inside any of the triangles
        if indices.len() >= 3 {
            for i in (0..indices.len()).step_by(3) {
                if i + 2 < indices.len() {
                    let v1 = (vertices[indices[i] as usize].position.x, vertices[indices[i] as usize].position.y);
                    let v2 =
                        (vertices[indices[i + 1] as usize].position.x, vertices[indices[i + 1] as usize].position.y);
                    let v3 =
                        (vertices[indices[i + 2] as usize].position.x, vertices[indices[i + 2] as usize].position.y);

                    if self.is_point_in_triangle(point, v1, v2, v3) {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn is_point_in_triangle(&self, p: (f32, f32), v1: (f32, f32), v2: (f32, f32), v3: (f32, f32)) -> bool {
        // Compute barycentric coordinates
        let d1 = self.sign(p, v1, v2);
        let d2 = self.sign(p, v2, v3);
        let d3 = self.sign(p, v3, v1);

        let has_neg = (d1 < 0.0) || (d2 < 0.0) || (d3 < 0.0);
        let has_pos = (d1 > 0.0) || (d2 > 0.0) || (d3 > 0.0);

        !(has_neg && has_pos)
    }

    fn sign(&self, p1: (f32, f32), p2: (f32, f32), p3: (f32, f32)) -> f32 {
        (p1.0 - p3.0) * (p2.1 - p3.1) - (p2.0 - p3.0) * (p1.1 - p3.1)
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
