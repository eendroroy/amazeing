use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

/// A maze generator/solver application with simulation/visualization.
#[derive(Debug, Clone, Parser)]
#[command(version, about)]
pub struct AmazeingArgs {
    #[clap(subcommand)]
    pub mode: ArgMode,

    /// Display size
    #[clap(global = true, long, value_name = "SIZE")]
    pub display_size: Option<ArgDisplaySize>,

    /// Display density
    #[clap(global = true, long, value_name = "DENSITY")]
    pub display_density: Option<ArgDisplayDensity>,

    /// Color scheme file (.toml) path
    #[clap(global = true, long, value_name = "SCHEME.TOML")]
    pub color_scheme: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Subcommand)]
pub enum ArgMode {
    /// Create a Maze
    #[clap(visible_alias = "C")]
    Create {
        /// File path to dump Maze data
        ///
        /// optional if '--simulate' flag provided
        ///
        /// if provided, generated maze will be dumped at path
        #[clap(long, required_unless_present = "simulate")]
        maze: Option<PathBuf>,

        /// Starting point of the generation
        #[clap(long, value_name = "usize,usize")]
        source: String,

        /// Maze Generation Procedure
        #[clap(long)]
        procedure: ArgGenProcedure,

        /// Number of rows
        #[clap(long)]
        rows: usize,

        /// Number of cols
        #[clap(long)]
        cols: usize,

        /// Show a simulation of the generation process
        #[clap(long, default_value_t = false)]
        simulate: bool,

        /// Frame update rate
        #[clap(long, default_value_t = 5)]
        fps: u8,

        /// View after generation
        #[clap(long, default_value_t = false)]
        view: bool,
    },
    /// View a Maze
    ///
    /// In modify ui Click MouseLeft a cell to open path and
    /// Shift+MouseLeft a cell to block path
    #[clap(visible_alias = "V")]
    View {
        /// Maze file path
        #[clap(long)]
        maze: PathBuf,

        /// View and update
        #[clap(long, default_value_t = false)]
        update: bool,
    },
    /// Solve a Maze
    ///
    /// In simulation ui click "MouseLeft" on a cell to select source
    /// and "Shift+MouseLeft" on a cell to select destination
    #[clap(visible_alias = "S")]
    Solve {
        /// Maze file path
        #[clap(long)]
        maze: PathBuf,

        /// Maze Solving Procedure
        #[clap(long)]
        procedure: ArgSolveProcedure,

        /// Heuristic function (to use with AStar)
        #[clap(long, required_if_eq("procedure", "a-star"))]
        heuristic_function: Option<ArgHeuristic>,

        /// Show a simulation of the solving process
        #[clap(long, default_value_t = false)]
        simulate: bool,

        /// Frame update rate
        #[clap(long, default_value_t = 5)]
        fps: u8,
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
    Random,
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
