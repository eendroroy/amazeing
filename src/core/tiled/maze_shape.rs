use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum MazeShape {
    Triangle,
    #[default]
    Rectangle,
    Hexagon,
    Circle,
}

impl MazeShape {
    pub fn as_str(&self) -> &'static str {
        match self {
            MazeShape::Triangle => "triangle",
            MazeShape::Rectangle => "rectangle",
            MazeShape::Hexagon => "hexagon",
            MazeShape::Circle => "circle",
        }
    }

    pub fn sides(&self) -> u8 {
        match self {
            MazeShape::Triangle => 3,
            MazeShape::Rectangle => 4,
            MazeShape::Hexagon => 6,
            MazeShape::Circle => 50,
        }
    }
}

impl Display for MazeShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for MazeShape {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "triangle" => Ok(MazeShape::Triangle),
            "rectangle" => Ok(MazeShape::Rectangle),
            "hexagon" => Ok(MazeShape::Hexagon),
            "circle" => Ok(MazeShape::Circle),
            _ => Err(format!("Unrecognized MazeShape: {}", s)),
        }
    }
}
