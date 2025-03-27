use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

/// A maze generator/solver application with simulation/visualization.
#[derive(Debug, Clone, Parser)]
#[command(version, about)]
pub struct AmazeingArgs {
    #[clap(subcommand)]
    pub mode: ArgMode,

    /// Display size
    #[clap(global = true, long, short = 'S', value_name = "SIZE")]
    pub display_size: Option<ArgDisplaySize>,

    /// Display density
    #[clap(global = true, long, short = 'D', value_name = "DENSITY")]
    pub display_density: Option<ArgDisplayDensity>,

    /// Color scheme file (.toml) path
    #[clap(global = true, short = 'C', long, value_name = "SCHEME.TOML")]
    pub color_scheme: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Subcommand)]
pub enum ArgMode {
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

        /// Starting point of the generation
        #[clap(long, short, value_name = "usize,usize")]
        source: String,

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
        #[clap(long, short, default_value_t = false, visible_alias = "verbose")]
        verbose: bool,

        /// Simulation speed
        #[clap(long, short, default_value_t = 5)]
        tempo: u8,
    },
    /// View a Maze
    ///
    /// In modify display_loop Click MouseLeft a cell to open path and
    /// Shift+MouseLeft a cell to block path
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
    ///
    /// In simulation display_loop click "MouseLeft" on a cell to select source
    /// and "Shift+MouseLeft" on a cell to select destination
    #[clap(visible_alias = "S")]
    Solve {
        /// Maze file path
        #[clap(long, short)]
        maze: PathBuf,

        /// Maze Solving Procedure
        #[clap(long, short)]
        procedure: ArgSolveProcedure,

        /// Heuristic function (to use with AStar)
        #[clap(long, short = 'H', required_if_eq("procedure", "a-star"))]
        heuristic_function: Option<ArgHeuristic>,

        /// Show a simulation of the generation process
        #[clap(long, short, default_value_t = false, visible_alias = "verbose")]
        verbose: bool,

        /// Simulation speed
        #[clap(long, short, default_value_t = 5)]
        tempo: u8,
    },
}

#[derive(Debug, Clone, PartialEq, ValueEnum)]
pub enum ArgDisplaySize {
    XXS,
    XS,
    S,
    M,
    L,
    XL,
    XXL,
}

#[derive(Debug, Clone, PartialEq, ValueEnum)]
pub enum ArgDisplayDensity {
    Connected,
    Dense,
    Standard,
    Cozy,
    Ample,
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
