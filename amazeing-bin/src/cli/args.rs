use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

/// A maze generator/solver application with simulation/visualization.
///
/// amazeing (generate | visualize | modify | solve)
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
    Visualize {
        /// Maze file path
        #[clap(long)]
        maze: PathBuf,
    },
    /// Modify a Maze
    ///
    /// Click MouseLeft a cell to open path and
    /// Shift+MouseLeft a cell to block path
    Modify {
        /// Maze file path
        #[clap(long)]
        maze: PathBuf,
    },
    /// Solve a Maze
    ///
    /// In simulation mode click "MouseLeft" on a cell select source
    /// and "Shift+MouseLeft" on a cell select destination
    Solve {
        // Show a simulation of the solving process
        #[clap(long, default_value_t = false)]
        simulate: bool,

        /// Maze file path
        #[clap(long)]
        maze: PathBuf,

        /// Source node to start simulation
        #[clap(long, value_name = "USIZE,USIZE", required_if_eq("simulate", "true"))]
        source: Option<String>,

        /// Destination node to stop simulation
        #[clap(long, value_name = "USIZE,USIZE", required_if_eq("simulate", "true"))]
        destination: Option<String>,

        /// Maze Solving Procedure
        #[clap(long)]
        procedure: ArgSolveProcedure,

        /// Heuristic function (to use with AStar)
        #[clap(long, required_if_eq("procedure", "a-star"))]
        heuristic_function: Option<ArgHeuristic>,

        /// Frame update rate
        #[clap(long)]
        fps: Option<u8>,
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
