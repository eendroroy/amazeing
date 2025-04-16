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

    /// Unit shape
    #[clap(
        global = true,
        long,
        short = 'U',
        display_order = 100,
        default_value_t = ArgUnitShape::Hexagon,
        value_name = "Shape"
    )]
    pub unit_shape: ArgUnitShape,

    /// Display size (zoom)
    #[clap(
        global = true,
        long,
        short = 'Z',
        display_order = 101,
        default_value_t = 1f32,
        value_name = "zoom"
    )]
    pub zoom: f32,

    /// Color file (.toml) path
    #[clap(global = true, long, short = 'C', display_order = 103, value_name = "Colors.toml")]
    pub colors: Option<PathBuf>,

    /// Frame rate per second (controls simulation speed)
    #[clap(global = true, long, short = 'F', display_order = 104, default_value_t = 60)]
    pub fps: u8,
}

#[derive(Debug, Clone, PartialEq, Subcommand)]
pub enum ArgCommand {
    /// Create a Maze
    #[clap(visible_alias = "C")]
    Create {
        /// File path to dump Maze data
        ///
        /// optional if '--verbose' flag provided
        ///
        /// if provided, generated maze will be dumped at path
        #[clap(long, short, required_unless_present = "verbose")]
        maze: Option<PathBuf>,

        /// Starting point(s) of the generation
        ///
        /// optional if '--verbose' flag provided
        #[clap(long, short, value_name = "usize,usize", required_unless_present = "verbose")]
        source: Option<Vec<String>>,

        /// Maze Generation Procedure
        #[clap(long, short, default_value_t = ArgGenProcedure::Dfs)]
        procedure: ArgGenProcedure,

        /// Number of rows
        #[clap(long, short)]
        rows: usize,

        /// Number of cols
        #[clap(long, short)]
        cols: usize,

        /// Show a simulation of the generation process
        #[clap(long, short, default_value_t = false)]
        verbose: bool,
    },
    /// View a Maze
    #[clap(visible_alias = "V")]
    View {
        /// Maze file path
        #[clap(long, short)]
        maze: PathBuf,

        /// View and update
        #[clap(long, short, default_value_t = false)]
        update: bool,
    },
    /// Solve a Maze
    #[clap(visible_alias = "S")]
    Solve {
        /// Maze file path
        #[clap(long, short)]
        maze: PathBuf,

        /// Maze Solving Procedure
        #[clap(long, short, default_value_t = ArgSolveProcedure::Dfs)]
        procedure: ArgSolveProcedure,

        /// Heuristic function (to use with AStar)
        #[clap(long, short = 'H', default_value_t = ArgHeuristic::Dijkstra, required_if_eq("procedure", "a-star"))]
        heuristic_function: ArgHeuristic,

        /// Show a simulation of the generation process
        #[clap(long, short, default_value_t = false, visible_alias = "verbose")]
        verbose: bool,
    },
}

#[derive(Debug, Clone, PartialEq, ValueEnum)]
pub enum ArgUnitShape {
    Triangle,
    Square,
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
