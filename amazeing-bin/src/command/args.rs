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
    pub display_size: Option<ArgDisplay>,

    /// Display scale (display size multiplier)
    #[clap(global = true, long, value_name = "SCALE")]
    pub display_scale: Option<f32>,

    /// Color scheme file (.toml) path
    #[clap(global = true, long, value_name = "SCHEME.TOML")]
    pub color_scheme: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Subcommand)]
pub enum ArgMode {
    /// Generate a Maze
    Generate {
        /// File path to dump Maze data
        #[clap(long)]
        maze: PathBuf,

        /// Maze Generation Procedure
        #[clap(long)]
        procedure: ArgGenProcedure,

        /// Number of rows
        #[clap(long)]
        rows: usize,

        /// Number of cols
        #[clap(long)]
        cols: usize,

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
pub enum ArgDisplay {
    XXS,
    XS,
    S,
    M,
    L,
    XL,
    XXL,
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
