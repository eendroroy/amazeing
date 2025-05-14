use crate::core::tiled::{BLOCK, Maze, MazeShape, Node, OPEN, UnitShape, VOID};
use crate::ui::component::MARGIN;
use crate::ui::component::unit_factory::{
    HexagonUnitShapeFactory, OctagonUnitShapeFactory, SquareUnitShapeFactory, TriangleUnitShapeFactory,
    UnitShapeFactory,
};
use crate::ui::context::Colors;
use crate::ui::helper::{current_millis, is_point_in_triangle};
use macroquad::prelude::YELLOW;
use macroquad::prelude::draw_line;
use macroquad::prelude::{Color, Vertex, vec2, vec3};
use macroquad::prelude::{Mesh, draw_mesh};
use std::f32::consts::PI;
use std::ops::{Index, IndexMut};

pub(crate) struct MazeScene {
    pub(crate) meshes: Vec<Vec<Mesh>>,
    pub(crate) maze: Maze,
    pub(crate) colors: Colors,
    pub(crate) dimension: (u32, u32),
    pub(crate) bound: Option<Mesh>,
    pub(crate) rows: usize,
    pub(crate) cols: usize,
    pub(crate) fps: f32,
    pub(crate) shape_factory: Box<dyn UnitShapeFactory>,
}

impl MazeScene {
    fn new_from(maze: &Maze, fps: f32, colors: &Colors, shape_factory: Box<dyn UnitShapeFactory>) -> Self {
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
            bound: None,
            rows: maze.rows(),
            cols: maze.cols(),
            fps,
            shape_factory,
        }
    }

    pub(crate) fn new_from_maze(maze: &Maze, zoom: f32, fps: f32, colors: &Colors) -> Self {
        let shape_factory = MazeScene::shape_factory(maze.unit_shape, zoom);
        MazeScene::new_from(maze, fps, colors, shape_factory)
    }

    pub(crate) fn new_from_dimension(
        maze_shape: MazeShape,
        unit_shape: UnitShape,
        rows: usize,
        cols: usize,
        zoom: f32,
        fps: f32,
        colors: &Colors,
    ) -> Self {
        let shape_factory = MazeScene::shape_factory(unit_shape, zoom);
        let (m_rows, m_cols) = MazeScene::adjust_dimension(maze_shape, unit_shape, rows, cols, &shape_factory);
        MazeScene::new_from(&Maze::new(maze_shape, unit_shape, m_rows, m_cols, VOID), fps, colors, shape_factory)
    }

    pub(crate) fn update(&mut self) {
        let node = Node::new(self.rows, self.cols);
        let color_open = self.colors.color_open;
        let color_block = self.colors.color_block;
        let color_bg = self.colors.color_bg;

        // Collect all updates first
        let updates: Vec<(Node, Color)> = self
            .maze
            .data
            .iter()
            .enumerate()
            .flat_map(|(row, row_data)| {
                row_data
                    .iter()
                    .enumerate()
                    .map(move |(col, &cell)| {
                        let color = match cell {
                            OPEN => color_open,
                            BLOCK => color_block,
                            VOID => color_bg,
                            _ => color_bg,
                        };
                        (node.at(row, col).unwrap(), color)
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        // Apply all updates without borrowing self.maze during the operation
        for (node, color) in updates {
            self.update_color(node, color);
        }
    }

    pub(crate) fn set_bound(&mut self, color: Color) {
        self.bound = Some(match (self.maze.maze_shape, self.maze.unit_shape) {
            // TODO: (MazeShape::Triangle, UnitShape::Triangle)
            (MazeShape::Triangle, _) => Mesh {
                #[rustfmt::skip]
                vertices: vec![
                    Vertex::new2(vec3((self.dimension.0 / 2) as f32         , MARGIN * 0.5                          , 0.), vec2(0., 0.), color),
                    Vertex::new2(vec3(self.dimension.0 as f32 - MARGIN * 0.5, self.dimension.1 as f32 - MARGIN * 0.5, 0.), vec2(0., 0.), color),
                    Vertex::new2(vec3(MARGIN * 0.5                          , self.dimension.1 as f32 - MARGIN * 0.5, 0.), vec2(0., 0.), color),
                ],
                indices: vec![0, 1, 2],
                texture: None,
            },
            (MazeShape::Rectangle, _) => Mesh {
                #[rustfmt::skip]
                vertices: vec![
                    Vertex::new2(vec3(MARGIN - 1.                          , MARGIN - 1.                          , 0.), vec2(0., 0.), color),
                    Vertex::new2(vec3(self.dimension.0 as f32 - MARGIN + 1., MARGIN - 1.                          , 0.), vec2(0., 0.), color),
                    Vertex::new2(vec3(self.dimension.0 as f32 - MARGIN + 1., self.dimension.1 as f32 - MARGIN + 1., 0.), vec2(0., 0.), color),
                    Vertex::new2(vec3(MARGIN - 1.                          , self.dimension.1 as f32 - MARGIN + 1., 0.), vec2(0., 0.), color),
                ],
                indices: vec![0, 1, 2, 0, 2, 3],
                texture: None,
            },
            (MazeShape::Circle | MazeShape::Hexagon, _) => {
                let sides = self.maze.maze_shape.sides();
                let radius = self.dimension.0 as f32 / 2.0 - MARGIN;
                let (x0, y0) = ((self.dimension.0 / 2) as f32, (self.dimension.1 / 2) as f32);
                let mut vertices = Vec::<Vertex>::with_capacity(sides as usize);
                let mut indices = vec![];
                for i in 0..sides {
                    let rx = (i as f32 / sides as f32 * PI * 2.).cos();
                    let ry = (i as f32 / sides as f32 * PI * 2.).sin();

                    let vertex = Vertex::new(x0 + radius * rx, y0 + radius * ry, 0., rx, ry, color);

                    vertices.push(vertex);

                    if i < sides - 2 {
                        indices.extend_from_slice(&[0, i as u16 + 1, i as u16 + 2]);
                    }
                }

                Mesh {
                    vertices,
                    indices,
                    texture: None,
                }
            }
        });

        let node = Node::new(self.rows, self.cols);

        for r in 0..self.rows {
            for c in 0..self.cols {
                if self.is_mesh_in_bound(&self.meshes[r][c]) {
                    self.update_color(node.at(r, c).unwrap(), self.colors.color_block);
                    self.maze[node.at(r, c).unwrap()] = BLOCK;
                }
            }
        }
    }

    pub(crate) fn draw(&self) {
        self.meshes.iter().for_each(|row| row.iter().for_each(draw_mesh));
    }

    pub(crate) fn draw_bound(&self) {
        if let Some(bound) = &self.bound {
            for i in 0..bound.vertices.len() {
                if i < bound.vertices.len() - 1 {
                    draw_line(
                        bound.vertices[i].position.x,
                        bound.vertices[i].position.y,
                        bound.vertices[i + 1].position.x,
                        bound.vertices[i + 1].position.y,
                        1.,
                        YELLOW,
                    )
                } else {
                    draw_line(
                        bound.vertices[i].position.x,
                        bound.vertices[i].position.y,
                        bound.vertices[0].position.x,
                        bound.vertices[0].position.y,
                        1.,
                        YELLOW,
                    )
                }
            }
        }
    }

    pub(crate) fn clicked_on(&self, (x, y): (f32, f32)) -> Option<Node> {
        for (row_idx, row) in self.meshes.iter().enumerate() {
            for (col_idx, mesh) in row.iter().enumerate() {
                if self.is_point_in_mesh(mesh, x, y) {
                    return Node::new(self.meshes.len() - 1, row.len() - 1).at(row_idx, col_idx);
                }
            }
        }
        None
    }

    pub(crate) fn update_color(&mut self, node: Node, color: Color) {
        self[node]
            .vertices
            .iter_mut()
            .for_each(|vertex| vertex.color = color.into())
    }

    pub(crate) fn delay_till_next_frame(&self, current_frame_start_time: u128) {
        let current_frame_time = (current_millis() - current_frame_start_time) as f32;
        let minimum_frame_time = (1. / self.fps) * 1000.;
        #[allow(unused_assignments)]
        let mut time_to_sleep: f32 = 0.;
        if current_frame_time < minimum_frame_time {
            time_to_sleep = minimum_frame_time - current_frame_time;
            std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));
        }
        // println!("Min: {}ms | Current: {}ms | Sleep: {}ms", minimum_frame_time, current_frame_time, time_to_sleep);
    }

    fn shape_factory(unit_shape: UnitShape, zoom: f32) -> Box<dyn UnitShapeFactory> {
        match unit_shape {
            UnitShape::Triangle => Box::new(TriangleUnitShapeFactory::new(zoom)),
            UnitShape::Square => Box::new(SquareUnitShapeFactory::new(zoom)),
            UnitShape::Hexagon => Box::new(HexagonUnitShapeFactory::new(zoom)),
            UnitShape::Octagon => Box::new(OctagonUnitShapeFactory::new(zoom)),
        }
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

    fn is_mesh_in_bound(&self, mesh: &Mesh) -> bool {
        // Return early if no bound is set
        let Some(bound) = &self.bound else {
            return false;
        };

        // Check if all vertices of the mesh are within the bound mesh
        // Calculate the center of the mesh
        let (sum_x, sum_y) = mesh
            .vertices
            .iter()
            .fold((0.0, 0.0), |acc, vertex| (acc.0 + vertex.position.x, acc.1 + vertex.position.y));
        let center_x = sum_x / mesh.vertices.len() as f32;
        let center_y = sum_y / mesh.vertices.len() as f32;
        let center_point = (center_x, center_y);

        // Check if the center point is inside any triangle of the bound mesh
        let bound_vertices = &bound.vertices;
        let bound_indices = &bound.indices;

        for i in (0..bound_indices.len()).step_by(3) {
            if i + 2 < bound_indices.len() {
                let v1 = (
                    bound_vertices[bound_indices[i] as usize].position.x,
                    bound_vertices[bound_indices[i] as usize].position.y,
                );
                let v2 = (
                    bound_vertices[bound_indices[i + 1] as usize].position.x,
                    bound_vertices[bound_indices[i + 1] as usize].position.y,
                );
                let v3 = (
                    bound_vertices[bound_indices[i + 2] as usize].position.x,
                    bound_vertices[bound_indices[i + 2] as usize].position.y,
                );

                if is_point_in_triangle(center_point, v1, v2, v3) {
                    return true;
                }
            }
        }

        false
    }

    fn adjust_dimension(
        maze_shape: MazeShape,
        unit_shape: UnitShape,
        rows: usize,
        cols: usize,
        factory: &Box<dyn UnitShapeFactory>,
    ) -> (usize, usize) {
        match (maze_shape, unit_shape) {
            (MazeShape::Rectangle, UnitShape::Triangle) => (rows * 2 + 1, cols),
            (MazeShape::Triangle, UnitShape::Triangle) => (rows * 2, if cols % 2 == 0 { cols + 1 } else { cols }),
            (MazeShape::Triangle, _) => (rows, if cols % 2 == 0 { cols + 1 } else { cols }),
            (MazeShape::Circle, UnitShape::Triangle) => {
                ((cols as f32 * factory.width() / factory.height()) as usize * 2, cols)
            }
            (MazeShape::Circle, _) => ((cols as f32 * factory.width() / factory.height()) as usize, cols),
            (_, _) => (rows, cols),
        }
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
