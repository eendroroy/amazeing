use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnitShape {
    Triangle,
    Square,
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

    pub fn from_str(str: &str) -> Self {
        match str {
            "triangle" => UnitShape::Triangle,
            "square" => UnitShape::Square,
            "hexagon" => UnitShape::Hexagon,
            "circle" => UnitShape::Circle,
            _ => panic!("Unrecognized shape: {}", str),
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

impl Default for UnitShape {
    fn default() -> Self {
        UnitShape::Hexagon
    }
}

impl Display for UnitShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

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