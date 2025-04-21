/// Represents the shape of the blocks of a tiled.
///
/// - `Square`: A shape with 4 sides, allowing movement in 4 directions.
/// - `Hexagon`: A shape with 6 sides, allowing movement in 6 directions.
#[derive(Debug, Clone, PartialEq)]
pub enum UnitShape {
    Triangle,
    Square,
    Hexagon,
    Circle,
}

impl UnitShape {
    /// Returns the number of sides or adjacent positions based on the shape.
    pub fn sides(&self) -> usize {
        match self {
            UnitShape::Triangle => 3,
            UnitShape::Square => 4,
            UnitShape::Hexagon => 6,
            UnitShape::Circle => 6,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MazeShape {
    Rectangle,
}
