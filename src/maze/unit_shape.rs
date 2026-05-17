use crate::maze::Node;
use crate::util::IsDivisible;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum UnitShape {
    Triangle,
    Square,
    Rhombus,
    #[default]
    Hexagon,
    HexagonRectangle,
    Octagon,
    OctagonSquare,
}

impl UnitShape {
    pub fn as_str(&self) -> &'static str {
        match self {
            UnitShape::Triangle => "triangle",
            UnitShape::Square => "square",
            UnitShape::Rhombus => "rhombus",
            UnitShape::Hexagon => "hexagon",
            UnitShape::HexagonRectangle => "hexagon-rectangle",
            UnitShape::Octagon => "octagon",
            UnitShape::OctagonSquare => "octagon-square",
        }
    }

    pub fn sides(&self, node: Node) -> usize {
        match self {
            UnitShape::Triangle => 3,
            UnitShape::Square => 4,
            UnitShape::Rhombus => 4,
            UnitShape::Hexagon => 6,
            UnitShape::HexagonRectangle => {
                if node.row.is_even() {
                    6
                } else {
                    4
                }
            }
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
            "rhombus" => Ok(UnitShape::Rhombus),
            "hexagon" => Ok(UnitShape::Hexagon),
            "hexagon-rectangle" => Ok(UnitShape::HexagonRectangle),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::maze::NodeFactory;

    #[test]
    fn unit_shape_string_roundtrip_works() {
        for shape in [
            UnitShape::Triangle,
            UnitShape::Square,
            UnitShape::Rhombus,
            UnitShape::Hexagon,
            UnitShape::HexagonRectangle,
            UnitShape::Octagon,
            UnitShape::OctagonSquare,
        ] {
            let s = shape.to_string();
            assert_eq!(UnitShape::from_str(&s).unwrap(), shape);
            assert_eq!(shape.as_str(), s);
        }
        assert!(UnitShape::from_str("unknown").is_err());
    }

    #[test]
    fn variable_side_shapes_depend_on_row_parity() {
        let f = NodeFactory::new(4, 4);
        let even = f.at(0, 0).unwrap();
        let odd = f.at(1, 0).unwrap();

        assert_eq!(UnitShape::HexagonRectangle.sides(even), 6);
        assert_eq!(UnitShape::HexagonRectangle.sides(odd), 4);
        assert_eq!(UnitShape::OctagonSquare.sides(even), 8);
        assert_eq!(UnitShape::OctagonSquare.sides(odd), 4);
        assert_eq!(UnitShape::Rhombus.sides(even), 4);
        assert_eq!(UnitShape::Rhombus.sides(odd), 4);
    }
}
