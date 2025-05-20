use crate::core::tiled::Node;
use crate::utility::IsDivisible;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum UnitShape {
    Triangle,
    Square,
    #[default]
    Hexagon,
    Octagon,
    OctagonSquare,
}

impl UnitShape {
    pub fn as_str(&self) -> &'static str {
        match self {
            UnitShape::Triangle => "triangle",
            UnitShape::Square => "square",
            UnitShape::Hexagon => "hexagon",
            UnitShape::Octagon => "octagon",
            UnitShape::OctagonSquare => "octagon-square",
        }
    }

    pub fn sides(&self, node: Node) -> usize {
        match self {
            UnitShape::Triangle => 3,
            UnitShape::Square => 4,
            UnitShape::Hexagon => 6,
            UnitShape::Octagon => 4,
            UnitShape::OctagonSquare => {
                if node.row.is_even() {
                    8
                } else {
                    4
                }
            }
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
            "octagon" => Ok(UnitShape::Octagon),
            "octagon-square" => Ok(UnitShape::OctagonSquare),
            _ => Err(format!("Unrecognized UnitShape: {}", s)),
        }
    }
}

impl Display for UnitShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
