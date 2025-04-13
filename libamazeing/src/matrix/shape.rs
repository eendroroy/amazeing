/// Represents the shape of the blocks of a matrix.
///
/// - `Square`: A shape with 4 sides, allowing movement in 4 directions.
/// - `Hexagon`: A shape with 6 sides, allowing movement in 6 directions.
#[derive(Debug, Clone, PartialEq)]
pub enum UnitShape {
    Square,
    Hexagon,
    Circle,
}

impl UnitShape {
    /// Returns the number of sides or adjacent positions based on the shape.
    pub fn sides(&self) -> usize {
        match self {
            UnitShape::Square => 4,
            UnitShape::Hexagon => 6,
            UnitShape::Circle => 6,
        }
    }
}
