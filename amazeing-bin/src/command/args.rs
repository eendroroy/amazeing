use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

/// A maze generator/solver application with simulation/visualization.
///
/// amazeing (generate | visualize | solve)
#[derive(Debug, Clone, Parser)]
#[command(version, about)]
pub struct AmazeingArgs {
    #[clap(subcommand)]
    pub mode: ArgMode,

    /// Display size
    #[clap(global = true, long, value_name = "SIZE")]
    pub display_size: Option<ArgDisplaySize>,

    /// Display density (changes space between adjacent blocks)
    #[clap(global = true, long, value_name = "DENSITY")]
    pub display_density: Option<ArgDisplayDensity>,

    /// Color scheme file (.toml) path
    #[clap(global = true, long, value_name = "SCHEME.TOML")]
    pub color_scheme: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Subcommand)]
pub enum ArgMode {
    /// Generate a Maze
    Generate {
        /// File path to dump Maze data
        ///
        /// optional if '--simulate' flag provided
        ///
        /// if provided, generated maze will be dumped at path
        #[clap(long, required_unless_present = "simulate")]
        maze: Option<PathBuf>,

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

        /// Visualize after generation
        #[clap(long, default_value_t = false)]
        visualize: bool,
    },
    /// Visualize a Maze
    ///
    /// In modify ui Click MouseLeft a cell to open path and
    /// Shift+MouseLeft a cell to block path
    Visualize {
        /// Maze file path
        #[clap(long)]
        maze: PathBuf,

        /// Visualize after generation
        #[clap(long, default_value_t = false)]
        modify: bool,
    },
    /// Solve a Maze
    ///
    /// In simulation ui click "MouseLeft" on a cell to select source
    /// and "Shift+MouseLeft" on a cell to select destination
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
    Stacked,
    Compact,
    Standard,
    Cozy,
    Spacious,
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

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    row: usize,
    col: usize,
}
