use clap::builder::Styles;
use clap::builder::styling::Color::Ansi;
use clap::builder::styling::{AnsiColor, Style};
use clap::{Parser, Subcommand, ValueEnum};
use std::fmt::Display;
use std::path::PathBuf;

pub const CLAP_STYLE: Styles = Styles::styled()
    .header(Style::new().bold().fg_color(Some(Ansi(AnsiColor::Green))))
    .usage(Style::new().bold().fg_color(Some(Ansi(AnsiColor::Green))))
    .literal(Style::new().fg_color(Some(Ansi(AnsiColor::Blue))).bold())
    .placeholder(Style::new().fg_color(Some(Ansi(AnsiColor::Cyan))))
    .error(Style::new().fg_color(Some(Ansi(AnsiColor::Red))).bold())
    .valid(Style::new().fg_color(Some(Ansi(AnsiColor::Green))))
    .invalid(Style::new().fg_color(Some(Ansi(AnsiColor::Yellow))));

/// A maze generator/solver application with simulation/visualization.
///
/// See https://eendroroy.github.io/amazeing for more details
#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about, styles=CLAP_STYLE)]
pub struct AmazeingArgs {
    #[clap(subcommand)]
    pub command: ArgCommand,

    /// Display size (zoom)
    #[clap(global = true, long, short = 'Z', display_order = 101, default_value_t = 1f32)]
    pub zoom: f32,

    /// Color file (.toml) path
    #[clap(global = true, long, short = 'C', display_order = 102, value_name = "Colors.toml")]
    pub colors: Option<PathBuf>,

    /// Frame rate per second (controls simulation speed)
    #[clap(global = true, long, short = 'F', display_order = 103, default_value_t = 60)]
    pub fps: u8,
}

/// {bin-name} amazeing create
#[derive(Debug, Clone, PartialEq, Subcommand)]
pub enum ArgCommand {
    /// Create a Maze
    #[clap(visible_alias = "C")]
    Create(CreateArgs),
    /// View a Maze
    #[clap(visible_alias = "V")]
    View(ViewArgs),
    /// Solve a Maze
    #[clap(visible_alias = "S")]
    Solve(SolveArgs),
}

#[derive(Debug, Clone, PartialEq, Parser)]
pub struct CreateArgs {
    /// Maze shape
    #[clap(subcommand)]
    pub maze_shape: ArgMazeShape,

    /// File path to dump Maze data
    ///
    /// optional if '--verbose' flag provided
    ///
    /// if provided, generated maze will be dumped at path
    #[clap(global = true, long, short)]
    pub maze: Option<PathBuf>,

    /// Show a simulation of the generation process
    #[clap(global = true, long, short, default_value_t = false)]
    pub verbose: bool,
}

#[derive(Debug, Clone, PartialEq, Parser)]
pub struct ViewArgs {
    /// Maze file path
    #[clap(long, short)]
    pub maze: PathBuf,

    /// View and update
    #[clap(long, short, default_value_t = false)]
    pub update: bool,
}

#[derive(Debug, Clone, PartialEq, Parser)]
pub struct SolveArgs {
    /// Maze file path
    #[clap(long, short)]
    pub maze: PathBuf,

    /// Maze Solving Procedure
    #[clap(long, short, default_value_t = ArgSolveProcedure::Dfs)]
    pub procedure: ArgSolveProcedure,

    /// Heuristic function (to use with AStar)
    #[clap(long, short = 'H', default_value_t = ArgHeuristic::Dijkstra, required_if_eq("procedure", "a-star"))]
    pub heuristic_function: ArgHeuristic,

    /// Show a simulation of the solving process
    #[clap(long, short, default_value_t = false)]
    pub verbose: bool,
}

#[derive(Debug, Clone, PartialEq, ValueEnum, Default)]
pub enum ArgUnitShape {
    Triangle,
    Square,
    #[default]
    Hexagon,
    Circle,
}

impl Display for ArgUnitShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgUnitShape::Triangle => write!(f, "triangle"),
            ArgUnitShape::Square => write!(f, "square"),
            ArgUnitShape::Hexagon => write!(f, "hexagon"),
            ArgUnitShape::Circle => write!(f, "circle"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Subcommand)]
pub enum ArgMazeShape {
    Rectangle(RectangleArgs),
}

impl Display for ArgMazeShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgMazeShape::Rectangle(_) => write!(f, "rectangle"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Parser)]
pub struct RectangleArgs {
    /// Unit shape
    #[clap(long, short, default_value_t = ArgUnitShape::default(), value_name = "UnitShape")]
    pub unit_shape: ArgUnitShape,

    /// Maze Generation Procedure
    #[clap(long, short, default_value_t = ArgGenProcedure::Dfs)]
    pub procedure: ArgGenProcedure,

    /// Number of rows
    #[clap(long, short)]
    pub rows: usize,

    /// Number of cols
    #[clap(long, short)]
    pub cols: usize,
}

#[derive(Debug, Clone, PartialEq, ValueEnum)]
pub enum ArgSolveProcedure {
    Bfs,
    Dfs,
    Dijkstra,
    AStar,
}

impl Display for ArgSolveProcedure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgSolveProcedure::Bfs => write!(f, "bfs"),
            ArgSolveProcedure::Dfs => write!(f, "dfs"),
            ArgSolveProcedure::Dijkstra => write!(f, "dijkstra"),
            ArgSolveProcedure::AStar => write!(f, "a-star"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, ValueEnum)]
pub enum ArgGenProcedure {
    Bfs,
    Dfs,
}

impl Display for ArgGenProcedure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgGenProcedure::Bfs => write!(f, "bfs"),
            ArgGenProcedure::Dfs => write!(f, "dfs"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, ValueEnum)]
pub enum ArgHeuristic {
    Manhattan,
    Euclidean,
    Chebyshev,
    Octile,
    Dijkstra,
}

impl Display for ArgHeuristic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgHeuristic::Manhattan => write!(f, "manhattan"),
            ArgHeuristic::Euclidean => write!(f, "euclidean"),
            ArgHeuristic::Chebyshev => write!(f, "chebyshev"),
            ArgHeuristic::Octile => write!(f, "octile"),
            ArgHeuristic::Dijkstra => write!(f, "dijkstra"),
        }
    }
}
