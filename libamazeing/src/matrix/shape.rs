/// Represents the shape of the blocks of a matrix.
///
/// - `Square`: A shape with 4 sides, allowing movement in 4 directions.
/// - `Hexagon`: A shape with 6 sides, allowing movement in 6 directions.
#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Square,
    Hexagon,
}

impl Shape {
    /// Returns the number of sides or adjacent positions based on the shape.
    pub fn sides(&self) -> usize {
        match self {
            Shape::Square => 4,
            Shape::Hexagon => 6,
        }
    }
}
