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
#[command(version, about, styles=CLAP_STYLE)]
pub struct AmazeingArgs {
    #[clap(subcommand)]
    pub command: ArgCommand,

    /// Block shape
    #[clap(global = true, long, short = 'B', default_value_t=ArgBlockShape::Square, value_name = "Shape")]
    pub block_shape: ArgBlockShape,

    /// Display size
    #[clap(global = true, long, short = 'S', default_value_t=ArgDisplaySize::M, value_name = "SIZE")]
    pub display_size: ArgDisplaySize,

    /// Display density
    #[clap(global = true, long, short = 'D', default_value_t=ArgDisplayDensity::Standard, value_name = "DENSITY")]
    pub display_density: ArgDisplayDensity,

    /// Color scheme file (.toml) path
    #[clap(global = true, short = 'C', long, value_name = "SCHEME.TOML")]
    pub color_scheme: Option<PathBuf>,
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
        #[clap(long, short)]
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

        /// Simulation speed
        #[clap(long, short, default_value_t = 60)]
        fps: u8,
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
        #[clap(long, short)]
        procedure: ArgSolveProcedure,

        /// Heuristic function (to use with AStar)
        #[clap(long, short = 'H', default_value_t=ArgHeuristic::Dijkstra, required_if_eq("procedure", "a-star"))]
        heuristic_function: ArgHeuristic,

        /// Show a simulation of the generation process
        #[clap(long, short, default_value_t = false, visible_alias = "verbose")]
        verbose: bool,

        /// Simulation speed
        #[clap(long, short, default_value_t = 5)]
        fps: u8,
    },
}

#[derive(Debug, Clone, PartialEq, ValueEnum)]
pub enum ArgDisplaySize {
    Xxs,
    Xs,
    S,
    M,
    L,
    Xl,
    Xxl,
}

impl Display for ArgDisplaySize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgDisplaySize::Xxs => write!(f, "xxs"),
            ArgDisplaySize::Xs => write!(f, "xs"),
            ArgDisplaySize::S => write!(f, "s"),
            ArgDisplaySize::M => write!(f, "m"),
            ArgDisplaySize::L => write!(f, "l"),
            ArgDisplaySize::Xl => write!(f, "xl"),
            ArgDisplaySize::Xxl => write!(f, "xxl"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, ValueEnum)]
pub enum ArgBlockShape {
    Square,
    Hexagon,
}

impl Display for ArgBlockShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgBlockShape::Square => write!(f, "square"),
            ArgBlockShape::Hexagon => write!(f, "hexagon"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, ValueEnum)]
pub enum ArgDisplayDensity {
    Connected,
    Dense,
    Standard,
    Cozy,
    Ample,
}

impl Display for ArgDisplayDensity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgDisplayDensity::Connected => write!(f, "connected"),
            ArgDisplayDensity::Dense => write!(f, "dense"),
            ArgDisplayDensity::Standard => write!(f, "standard"),
            ArgDisplayDensity::Cozy => write!(f, "cozy"),
            ArgDisplayDensity::Ample => write!(f, "ample"),
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

#[derive(Debug, Clone, PartialEq, ValueEnum)]
pub enum ArgGenProcedure {
    Bfs,
    Dfs,
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
