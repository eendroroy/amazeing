use crate::cli::{AmazeingContext, Colors, ContextType};
use crate::maze::{BLOCK, Maze, Node, NodeFactory, OPEN, Rank, UnitShape};
use crate::render::helper::{current_micros, is_point_in_triangle};
use crate::render::unit::{
    HexagonRectangleUnitShapeFactory, HexagonUnitShapeFactory, OctagonSquareUnitShapeFactory, OctagonUnitShapeFactory,
    RhombusUnitShapeFactory, SquareUnitShapeFactory, TriangleUnitShapeFactory, UnitShapeFactory,
};
use crate::render::{BORDER, MARGIN};
use crate::util::IsDivisible;
use macroquad::prelude::{Color, Mesh, Vertex, clear_background, draw_line, draw_mesh, vec2, vec3};

// ── constants ─────────────────────────────────────────────────────────────────

/// macroquad's internal geometry batcher (quad_gl) rejects calls that exceed
/// these limits and prints "geometry() exceeded max drawcall size, clamping".
/// We stay a little below the hard limits to leave headroom.
///
/// Limits from macroquad 0.4 `src/quad_gl.rs`:
///   MAX_VERTICES = 10_000
///   MAX_INDICES  =  5_000
const CHUNK_MAX_VERTS: usize = 9_900;
const CHUNK_MAX_INDICES: usize = 4_950;

// ── internal data types ───────────────────────────────────────────────────────

/// Location of a single cell's vertices inside the flat `scene_chunks` array.
#[derive(Clone, Copy)]
struct CellLocation {
    /// Index into `scene_chunks`.
    chunk: usize,
    /// First vertex index inside `scene_chunks[chunk].vertices`.
    vertex_start: usize,
    /// Number of vertices belonging to this cell.
    vertex_count: usize,
}

/// Polygon vertex positions used only for hit-testing.
/// No colour data is stored here; all colour lives in `scene_chunks`.
struct CellHitData {
    positions: Vec<(f32, f32)>,
}

// ── MazeScene ─────────────────────────────────────────────────────────────────

pub(crate) struct MazeScene {
    pub(crate) context: AmazeingContext,
    pub(crate) maze: Maze,
    pub(crate) colors: Colors,
    pub(crate) wh: (u32, u32),
    pub(crate) bound: Option<Mesh>,
    pub(crate) rows: usize,
    pub(crate) cols: usize,

    /// Flat rendering geometry.
    /// All cell polygons are merged into a small number of large Mesh objects
    /// (typically 3-7 for a 175×285 maze) so that `draw()` issues only that
    /// many GPU draw calls instead of one per cell (up to ~50 000).
    scene_chunks: Vec<Mesh>,

    /// For each cell: which chunk it lives in and its vertex range.
    cell_locations: Vec<Vec<CellLocation>>,

    /// Lightweight per-cell geometry used only for click detection.
    cell_hitdata: Vec<Vec<CellHitData>>,
}

// ── construction helpers ──────────────────────────────────────────────────────

#[inline]
fn cell_color(cell: i8, colors: &Colors) -> Color {
    match cell {
        OPEN => colors.color_open,
        BLOCK => colors.color_block,
        _ => colors.color_bg,
    }
}

/// Build flat scene chunks, per-cell location table, and hit-test data
/// in a single pass over the maze.
fn build_scene(
    maze: &Maze,
    shape_factory: &dyn UnitShapeFactory,
    colors: &Colors,
) -> (Vec<Mesh>, Vec<Vec<CellLocation>>, Vec<Vec<CellHitData>>) {
    let (rows, cols) = (maze.rows(), maze.cols());

    let mut chunks: Vec<Mesh> = vec![Mesh {
        vertices: Vec::new(),
        indices: Vec::new(),
        texture: None,
    }];
    let mut locations: Vec<Vec<CellLocation>> = Vec::with_capacity(rows);
    let mut hitdata: Vec<Vec<CellHitData>> = Vec::with_capacity(rows);

    for r in 0..rows {
        let mut row_locs = Vec::with_capacity(cols);
        let mut row_hits = Vec::with_capacity(cols);

        for c in 0..cols {
            let color = cell_color(maze.data[r][c], colors);
            let mesh = shape_factory.shape(r, c, rows, cols, color);

            // Collect vertex positions (read-only; used only for hit testing).
            let positions = mesh.vertices.iter().map(|v| (v.position.x, v.position.y)).collect();
            row_hits.push(CellHitData { positions });

            // Pack this cell into the current chunk, starting a new chunk when
            // either the vertex count or index count would exceed macroquad's
            // internal geometry batcher limits.
            let vcount = mesh.vertices.len();
            let icount = mesh.indices.len();
            {
                let last = chunks.last().unwrap();
                if last.vertices.len() + vcount > CHUNK_MAX_VERTS || last.indices.len() + icount > CHUNK_MAX_INDICES {
                    chunks.push(Mesh {
                        vertices: Vec::new(),
                        indices: Vec::new(),
                        texture: None,
                    });
                }
            }

            let chunk_idx = chunks.len() - 1;
            let chunk = chunks.last_mut().unwrap();
            let vstart = chunk.vertices.len();
            let base = vstart as u16;

            chunk.vertices.extend_from_slice(&mesh.vertices);
            for &idx in &mesh.indices {
                chunk.indices.push(base + idx);
            }

            row_locs.push(CellLocation {
                chunk: chunk_idx,
                vertex_start: vstart,
                vertex_count: vcount,
            });
        }

        locations.push(row_locs);
        hitdata.push(row_hits);
    }

    (chunks, locations, hitdata)
}

// ── MazeScene impl ───────────────────────────────────────────────────────────

impl MazeScene {
    fn new_from(
        maze: &Maze,
        context: AmazeingContext,
        colors: &Colors,
        shape_factory: Box<dyn UnitShapeFactory>,
    ) -> Self {
        let wh = shape_factory.wh_for(maze.rows(), maze.cols());
        let (scene_chunks, cell_locations, cell_hitdata) = build_scene(maze, shape_factory.as_ref(), colors);

        let mut scene = Self {
            context,
            maze: maze.clone(),
            colors: colors.clone(),
            wh,
            bound: None,
            rows: maze.rows(),
            cols: maze.cols(),
            scene_chunks,
            cell_locations,
            cell_hitdata,
        };

        if scene.context.context_type == ContextType::Create || scene.context.show_perimeter {
            scene.set_bound();
        }

        scene
    }

    pub(crate) fn new_from_maze(maze: &Maze, context: &AmazeingContext, colors: &Colors) -> Self {
        let shape_factory = MazeScene::shape_factory(maze.unit_shape, context.zoom);
        MazeScene::new_from(maze, context.clone(), colors, shape_factory)
    }

    pub(crate) fn new_from_dimension(unit_shape: UnitShape, context: &AmazeingContext, colors: &Colors) -> Self {
        let shape_factory = MazeScene::shape_factory(unit_shape, context.zoom);
        let (m_rows, m_cols) = MazeScene::adjust_dimension(unit_shape, context.rows, context.cols);
        MazeScene::new_from(&Maze::new_void(unit_shape, m_rows, m_cols), context.clone(), colors, shape_factory)
    }

    pub(crate) fn w(&self) -> u32 {
        self.wh.0
    }
    pub(crate) fn h(&self) -> u32 {
        self.wh.1
    }

    /// Bulk-refresh all cell colours from the current maze state.
    /// Called after a generation/load that replaced the entire maze.
    pub(crate) fn update(&mut self) {
        let color_open = self.colors.color_open;
        let color_block = self.colors.color_block;
        let color_bg = self.colors.color_bg;

        for r in 0..self.rows {
            for c in 0..self.cols {
                let color = match self.maze.data[r][c] {
                    OPEN => color_open,
                    BLOCK => color_block,
                    _ => color_bg,
                };
                let loc = self.cell_locations[r][c];
                let col: [u8; 4] = color.into();
                let start = loc.vertex_start;
                let end = start + loc.vertex_count;
                for v in &mut self.scene_chunks[loc.chunk].vertices[start..end] {
                    v.color = col;
                }
            }
        }
    }

    pub(crate) fn set_bound(&mut self) {
        let (x_min, x_max) = (MARGIN - BORDER, self.w() as f32 - MARGIN + BORDER);
        let (y_min, y_max) = (MARGIN - BORDER, self.h() as f32 - MARGIN + BORDER);
        self.bound = Some(Mesh {
            vertices: vec![
                Vertex::new2(vec3(x_min, y_min, 0.), vec2(0., 0.), self.colors.color_perimeter),
                Vertex::new2(vec3(x_max, y_min, 0.), vec2(0., 0.), self.colors.color_perimeter),
                Vertex::new2(vec3(x_max, y_max, 0.), vec2(0., 0.), self.colors.color_perimeter),
                Vertex::new2(vec3(x_min, y_max, 0.), vec2(0., 0.), self.colors.color_perimeter),
            ],
            indices: vec![0, 1, 2, 0, 2, 3],
            texture: None,
        });

        if self.context.context_type == ContextType::Create {
            // Determine which cells fall inside the bound before mutating colours
            // (avoids a simultaneous immutable+mutable borrow of self).
            let in_bound: Vec<(usize, usize)> = {
                let bound = self.bound.as_ref().unwrap();
                let mut result = Vec::new();
                for r in 0..self.rows {
                    for c in 0..self.cols {
                        if Self::cell_in_bound(bound, &self.cell_hitdata[r][c]) {
                            result.push((r, c));
                        }
                    }
                }
                result
            };

            let node_factory = NodeFactory::new(self.rows, self.cols);
            let color_block = self.colors.color_block;
            for (r, c) in in_bound {
                let node = node_factory.at(r, c).unwrap();
                self.update_color(node, color_block);
                self.maze[node] = BLOCK;
            }
        }
    }

    pub(crate) fn clear_and_draw(&self) {
        clear_background(self.colors.color_bg);
        self.draw();
        self.draw_bound();
    }

    /// Draw the entire scene.  Issues one `draw_mesh` call per chunk —
    /// typically 3-7 calls regardless of maze size.
    pub(crate) fn draw(&self) {
        self.scene_chunks.iter().for_each(draw_mesh);
    }

    pub(crate) fn draw_bound(&self) {
        if !self.context.show_perimeter {
            return;
        }
        if let Some(bound) = &self.bound {
            let verts = &bound.vertices;
            let n = verts.len();
            for i in 0..n {
                let next = (i + 1) % n;
                draw_line(
                    verts[i].position.x,
                    verts[i].position.y,
                    verts[next].position.x,
                    verts[next].position.y,
                    1.,
                    self.colors.color_perimeter,
                );
            }
        }
    }

    pub(crate) fn clicked_on(&self, (x, y): (f32, f32)) -> Option<Node> {
        for (row_idx, row) in self.cell_hitdata.iter().enumerate() {
            for (col_idx, hit) in row.iter().enumerate() {
                if Self::is_point_in_hit(hit, x, y) {
                    return NodeFactory::new(self.rows, self.cols).at(row_idx, col_idx);
                }
            }
        }
        None
    }

    pub(crate) fn update_node(&mut self, node: Node, value: i8, color: Color) {
        self.update_color(node, color);
        self.maze[node] = value;
    }

    /// Update vertex colours for a single cell in the appropriate scene chunk.
    /// O(vertices_per_cell) — typically 3-8 vertex writes.
    pub(crate) fn update_color(&mut self, node: Node, color: Color) {
        let loc = self.cell_locations[node.row][node.col];
        let c: [u8; 4] = color.into();
        let start = loc.vertex_start;
        let end = start + loc.vertex_count;
        for v in &mut self.scene_chunks[loc.chunk].vertices[start..end] {
            v.color = c;
        }
    }

    #[allow(dead_code)]
    pub(crate) fn display_void(&mut self, node: Node) {
        self.update_color(node, self.colors.color_bg)
    }

    pub(crate) fn display_block(&mut self, node: Node) {
        self.update_color(node, self.colors.color_block)
    }

    pub(crate) fn display_open(&mut self, node: Node) {
        self.update_color(node, self.colors.color_open)
    }

    #[allow(dead_code)]
    pub(crate) fn display_visiting(&mut self, node: Node) {
        self.update_color(node, self.colors.color_visiting)
    }

    pub(crate) fn display_visiting_gradient(&mut self, node: Node, rank: &Rank) {
        self.update_color(node, *self.colors.shed_color(rank))
    }

    pub(crate) fn display_path(&mut self, node: Node) {
        self.update_color(node, self.colors.color_path)
    }

    pub(crate) fn display_source(&mut self, node: Node) {
        self.update_color(node, self.colors.color_source)
    }

    pub(crate) fn display_destination(&mut self, node: Node) {
        self.update_color(node, self.colors.color_destination)
    }

    pub(crate) fn display_traversed(&mut self, node: Node) {
        self.update_color(node, self.colors.color_traversed)
    }

    /// Sleep until the next frame deadline using microsecond precision.
    /// Allows accurate pacing at any FPS value including values above 300.
    pub(crate) fn delay_till_next_frame(&self, frame_start_us: u128) {
        let elapsed_us = (current_micros() - frame_start_us) as f64;
        let target_us = 1_000_000.0 / self.context.fps as f64;
        if elapsed_us < target_us {
            let sleep_us = (target_us - elapsed_us) as u64;
            if sleep_us > 0 {
                std::thread::sleep(std::time::Duration::from_micros(sleep_us));
            }
        }
    }

    // ── private helpers ───────────────────────────────────────────────────────

    fn shape_factory(unit_shape: UnitShape, zoom: f32) -> Box<dyn UnitShapeFactory> {
        match unit_shape {
            UnitShape::Triangle => Box::new(TriangleUnitShapeFactory::new(zoom)),
            UnitShape::Square => Box::new(SquareUnitShapeFactory::new(zoom)),
            UnitShape::Rhombus => Box::new(RhombusUnitShapeFactory::new(zoom)),
            UnitShape::Hexagon => Box::new(HexagonUnitShapeFactory::new(zoom)),
            UnitShape::HexagonRectangle => Box::new(HexagonRectangleUnitShapeFactory::new(zoom)),
            UnitShape::Octagon => Box::new(OctagonUnitShapeFactory::new(zoom)),
            UnitShape::OctagonSquare => Box::new(OctagonSquareUnitShapeFactory::new(zoom)),
        }
    }

    /// Triangle-fan hit test for a convex polygon stored in `hit.positions`.
    fn is_point_in_hit(hit: &CellHitData, x: f32, y: f32) -> bool {
        let pos = &hit.positions;
        let n = pos.len();
        if n < 3 {
            return false;
        }
        for i in 1..n - 1 {
            if is_point_in_triangle((x, y), pos[0], pos[i], pos[i + 1]) {
                return true;
            }
        }
        false
    }

    /// Returns `true` when the centroid of `hit` lies inside the bound mesh.
    fn cell_in_bound(bound: &Mesh, hit: &CellHitData) -> bool {
        let n = hit.positions.len();
        if n == 0 {
            return false;
        }
        let (sx, sy) = hit
            .positions
            .iter()
            .fold((0.0f32, 0.0f32), |acc, &(x, y)| (acc.0 + x, acc.1 + y));
        let center = (sx / n as f32, sy / n as f32);

        let bv = &bound.vertices;
        let bi = &bound.indices;
        for i in (0..bi.len()).step_by(3) {
            if i + 2 < bi.len() {
                let v1 = (bv[bi[i] as usize].position.x, bv[bi[i] as usize].position.y);
                let v2 = (bv[bi[i + 1] as usize].position.x, bv[bi[i + 1] as usize].position.y);
                let v3 = (bv[bi[i + 2] as usize].position.x, bv[bi[i + 2] as usize].position.y);
                if is_point_in_triangle(center, v1, v2, v3) {
                    return true;
                }
            }
        }
        false
    }

    fn adjust_dimension(unit_shape: UnitShape, rows: usize, cols: usize) -> (usize, usize) {
        match unit_shape {
            UnitShape::Triangle | UnitShape::HexagonRectangle | UnitShape::OctagonSquare => {
                ((rows * 2).odd_floor(), cols)
            }
            _ => (rows, cols),
        }
    }
}
