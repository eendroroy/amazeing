use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum MazeShape {
    Triangle,
    #[default]
    Rectangle,
    Square,
    Circle,
}

impl MazeShape {
    pub fn as_str(&self) -> &'static str {
        match self {
            MazeShape::Triangle => "triangle",
            MazeShape::Rectangle => "rectangle",
            MazeShape::Square => "square",
            MazeShape::Circle => "circle",
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
            "square" => Ok(MazeShape::Square),
            "circle" => Ok(MazeShape::Circle),
            _ => Err(format!("Unrecognized MazeShape: {}", s)),
        }
    }
}
