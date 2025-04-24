use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum UnitShape {
    Triangle,
    Square,
    #[default]
    Hexagon,
    Circle,
}

impl UnitShape {
    pub fn as_str(&self) -> &'static str {
        match self {
            UnitShape::Triangle => "triangle",
            UnitShape::Square => "square",
            UnitShape::Hexagon => "hexagon",
            UnitShape::Circle => "circle",
        }
    }

    pub fn sides(&self) -> usize {
        match self {
            UnitShape::Triangle => 3,
            UnitShape::Square => 4,
            UnitShape::Hexagon => 6,
            UnitShape::Circle => 6,
        }
    }
}

impl FromStr for UnitShape {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "triangle" => Ok(UnitShape::Triangle),
            "square" => Ok(UnitShape::Square),
            "hexagon" => Ok(UnitShape::Hexagon),
            "circle" => Ok(UnitShape::Circle),
            _ => Err(format!("Unrecognized UnitShape: {}", s)),
        }
    }
}

impl Display for UnitShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
