use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MazeShape {
    Rectangle,
}

impl MazeShape {
    pub fn as_str(&self) -> &'static str {
        match self {
            MazeShape::Rectangle => "rectangle",
        }
    }

    pub fn from_str(str: &str) -> Self {
        match str {
            "rectangle" => MazeShape::Rectangle,
            _ => panic!("Unrecognized shape: {}", str),
        }
    }
}

impl Default for MazeShape {
    fn default() -> Self {
        MazeShape::Rectangle
    }
}

impl Display for MazeShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
