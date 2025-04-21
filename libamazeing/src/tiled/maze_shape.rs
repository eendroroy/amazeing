use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum MazeShape {
    #[default]
    Rectangle,
}

impl MazeShape {
    pub fn as_str(&self) -> &'static str {
        match self {
            MazeShape::Rectangle => "rectangle",
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
            "rectangle" => Ok(MazeShape::Rectangle),
            _ => Err(format!("Unrecognized MazeShape: {}", s)),
        }
    }
}
