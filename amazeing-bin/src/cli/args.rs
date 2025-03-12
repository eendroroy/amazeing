use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

/// A maze generator/solver application with simulation/visualization.
#[derive(Debug, Clone, Parser)]
#[command(version, about)]
pub struct Arg {
    #[clap(subcommand)]
    pub mode: ArgMode,

    /// Display size
    #[clap(long)]
    pub display_size: Option<ArgDisplay>,

    /// Display size multiplier
    #[clap(long)]
    pub display_scale: Option<f32>,

    /// Color scheme file path
    #[clap(long)]
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
    },
    /// Visualize a Maze
    Visualize {
        /// Maze file path
        #[clap(long)]
        maze: PathBuf,
    },
    /// Modify a Maze
    Modify {
        /// Maze file path
        #[clap(long)]
        maze: PathBuf,
    },
    /// Simulation of Maze solver
    Simulate {
        /// Maze file path
        #[clap(long)]
        maze: PathBuf,

        /// Source node (`"usize,usize"`) to start simulation
        #[clap(long)]
        source: String,

        /// Destination node (`"usize,usize"`) to stop simulation
        #[clap(long)]
        destination: String,

        /// Maze Solving Procedure
        #[clap(long)]
        procedure: ArgSolveProcedure,

        /// Heuristic function (to use with AStar)
        #[clap(long)]
        #[arg(required_if_eq("procedure", "a-star"))]
        heuristic: Option<ArgHeuristic>,

        /// Frame update rate
        #[clap(long)]
        fps: Option<u8>,
    },
    /// Realtime path find in a Maze
    Realtime {
        /// Maze file path
        #[clap(long)]
        maze: PathBuf,

        /// Maze Solving Procedure
        #[clap(long)]
        procedure: ArgSolveProcedure,

        /// Heuristic function (to use with AStar)
        #[clap(long)]
        #[arg(required_if_eq("procedure", "a-star"))]
        heuristic: Option<ArgHeuristic>,
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

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    row: usize,
    col: usize,
}
