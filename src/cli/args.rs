use clap::builder::Styles;
use clap::builder::styling::Color::Ansi;
use clap::builder::styling::{
    AnsiColor::{Blue, Cyan, Green, Red, Yellow},
    Style,
};
use clap::{Parser, Subcommand, ValueEnum};
use std::fmt::Display;
use std::path::PathBuf;

pub const CLAP_STYLE: Styles = Styles::styled()
    .header(Style::new().bold().fg_color(Some(Ansi(Green))))
    .usage(Style::new().bold().fg_color(Some(Ansi(Green))))
    .literal(Style::new().fg_color(Some(Ansi(Blue))).bold())
    .placeholder(Style::new().fg_color(Some(Ansi(Cyan))))
    .error(Style::new().fg_color(Some(Ansi(Red))).bold())
    .valid(Style::new().fg_color(Some(Ansi(Green))))
    .invalid(Style::new().fg_color(Some(Ansi(Yellow))));

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

    /// Draw maze bound (perimeter)
    #[clap(global = true, long, short = 'P', display_order = 103, default_value_t = false)]
    pub show_perimeter: bool,
}

/// {ui-name} amazeing create
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
    /// Unit shape
    #[clap(global = true, long, short, default_value_t = ArgUnitShape::default(), value_name = "UnitShape")]
    pub unit_shape: ArgUnitShape,

    /// File path to dump Maze data
    ///
    /// optional if '--verbose' flag provided
    ///
    /// if provided, generated maze will be dumped at path
    #[clap(global = true, long, short)]
    pub maze: Option<PathBuf>,

    /// Number of rows
    #[clap(long, short)]
    pub rows: usize,

    /// Number of columns
    #[clap(long, short)]
    pub cols: usize,

    /// Maze Generation Procedure
    #[clap(global = true, long, short, default_value_t = ArgProcedure::Dfs)]
    pub procedure: ArgProcedure,

    /// Heuristic function (to use with AStar)
    #[clap(global = true, long, short = 'H', default_value_t = ArgHeuristic::Dijkstra, required_if_eq("procedure", "a-star"))]
    pub heuristic_function: ArgHeuristic,

    /// Weight randomization factor (to use with AStar)
    #[clap(global = true, long, short, default_value_t = 2)]
    pub jumble_factor: u32,

    /// Weight direction (ordering) (to use with AStar)
    #[clap(global = true, long, short, default_value_t = ArgWeightDirection::default())]
    pub weight_direction: ArgWeightDirection,

    /// Show a simulation of the generation process
    #[clap(global = true, long, short, default_value_t = false)]
    pub verbose: bool,

    /// Frame rate per second (controls simulation speed); only valid with --verbose
    #[clap(long, short = 'F', display_order = 103, default_value_t = 60., requires = "verbose")]
    pub fps: f32,

    /// Visual effect to apply during simulation; only valid with --verbose
    #[clap(long, short = 'E', display_order = 104, value_name = "Effect", action = clap::ArgAction::Append, requires = "verbose")]
    pub effect: Vec<ArgEffect>,
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
    #[clap(long, short, default_value_t = ArgProcedure::Dfs)]
    pub procedure: ArgProcedure,

    /// Heuristic function (to use with AStar)
    #[clap(long, short = 'H', default_value_t = ArgHeuristic::default(), required_if_eq("procedure", "a-star"))]
    pub heuristic_function: ArgHeuristic,

    /// Show a simulation of the solving process
    #[clap(long, short, default_value_t = false)]
    pub verbose: bool,

    /// Frame rate per second (controls simulation speed); only valid with --verbose
    #[clap(long, short = 'F', display_order = 103, default_value_t = 60., requires = "verbose")]
    pub fps: f32,

    /// Visual effect to apply during simulation; only valid with --verbose
    #[clap(long, short = 'E', display_order = 104, value_name = "Effect", action = clap::ArgAction::Append, requires = "verbose")]
    pub effect: Vec<ArgEffect>,
}

#[derive(Debug, Clone, PartialEq, ValueEnum, Default)]
pub enum ArgUnitShape {
    #[clap(alias = "t")]
    Triangle,
    #[clap(alias = "s")]
    Square,
    #[clap(alias = "r")]
    Rhombus,
    #[default]
    #[clap(alias = "h")]
    Hexagon,
    #[clap(alias = "hr")]
    HexagonRectangle,
    #[clap(alias = "o")]
    Octagon,
    #[clap(alias = "os")]
    OctagonSquare,
}

impl Display for ArgUnitShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgUnitShape::Triangle => write!(f, "triangle"),
            ArgUnitShape::Square => write!(f, "square"),
            ArgUnitShape::Rhombus => write!(f, "rhombus"),
            ArgUnitShape::Hexagon => write!(f, "hexagon"),
            ArgUnitShape::HexagonRectangle => write!(f, "hexagon-rectangle"),
            ArgUnitShape::Octagon => write!(f, "octagon"),
            ArgUnitShape::OctagonSquare => write!(f, "octagon-square"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, ValueEnum, Default)]
pub enum ArgWeightDirection {
    #[clap(alias = "f")]
    Forward,
    #[clap(alias = "b")]
    #[default]
    Backward,
}

impl Display for ArgWeightDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgWeightDirection::Forward => write!(f, "forward"),
            ArgWeightDirection::Backward => write!(f, "backward"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, ValueEnum, Default)]
pub enum ArgProcedure {
    #[clap(alias = "b")]
    Bfs,
    #[default]
    #[clap(alias = "d")]
    Dfs,
    #[clap(alias = "p")]
    Prim,
    #[clap(alias = "i")]
    Iddfs,
    #[clap(alias = "gbf")]
    GreedyBestFirst,
    #[clap(alias = "bb")]
    BidirectionalBfs,
    #[clap(alias = "bs")]
    BeamSearch,
    #[clap(alias = "bgbf")]
    BidirectionalGreedyBestFirst,
    #[clap(alias = "sas")]
    SimulatedAnnealingSearch,
    #[clap(alias = "ab")]
    AldousBroder,
    #[clap(alias = "ba")]
    BidirectionalAStart,
    #[clap(alias = "a")]
    AStar,
}

impl Display for ArgProcedure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgProcedure::Bfs => write!(f, "bfs"),
            ArgProcedure::Dfs => write!(f, "dfs"),
            ArgProcedure::Prim => write!(f, "prim"),
            ArgProcedure::Iddfs => write!(f, "iddfs"),
            ArgProcedure::GreedyBestFirst => write!(f, "greedy-best-first"),
            ArgProcedure::BidirectionalBfs => write!(f, "bidirectional-bfs"),
            ArgProcedure::BeamSearch => write!(f, "beam-search"),
            ArgProcedure::BidirectionalGreedyBestFirst => write!(f, "bidirectional-greedy-best-first"),
            ArgProcedure::SimulatedAnnealingSearch => write!(f, "simulated-annealing-search"),
            ArgProcedure::AldousBroder => write!(f, "aldous-broder"),
            ArgProcedure::BidirectionalAStart => write!(f, "bidirectional-a-start"),
            ArgProcedure::AStar => write!(f, "a-star"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, ValueEnum, Default)]
pub enum ArgHeuristic {
    #[clap(alias = "m")]
    Manhattan,
    #[clap(alias = "e")]
    Euclidean,
    #[clap(alias = "c")]
    Chebyshev,
    #[clap(alias = "o")]
    Octile,
    /// Exact minimum-step distance for hexagonal (odd-r offset) grids.
    /// Use this with `--unit-shape hexagon` or `hexagon-rectangle`.
    #[clap(alias = "x")]
    Hex,
    #[default]
    #[clap(alias = "d")]
    Dijkstra,
}

impl Display for ArgHeuristic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgHeuristic::Manhattan => write!(f, "manhattan"),
            ArgHeuristic::Euclidean => write!(f, "euclidean"),
            ArgHeuristic::Chebyshev => write!(f, "chebyshev"),
            ArgHeuristic::Octile => write!(f, "octile"),
            ArgHeuristic::Hex => write!(f, "hex"),
            ArgHeuristic::Dijkstra => write!(f, "dijkstra"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, ValueEnum)]
pub enum ArgEffect {
    /// Torch-light effect: the visiting frontier illuminates nearby cells,
    /// with brightness falling off as distance grows.
    #[clap(alias = "ls")]
    LightSource,
    /// Fish-eye zoom effect: the visiting frontier is magnified; the zoom
    /// falls off smoothly as grid-cell distance grows.
    #[clap(alias = "fe")]
    FishEye,
    /// Color-source effect: the visiting-peak color bleeds / glows onto nearby
    /// cells, with the tint intensity falling off smoothly with distance.
    #[clap(alias = "cs")]
    ColorSource,
    /// Shockwave distortion effect: an animated radial ripple emanates from the
    /// visiting frontier, displacing vertices in a sine-wave pattern that decays
    /// exponentially with distance.
    #[clap(alias = "sw")]
    ShockwaveDistortion,
}

impl Display for ArgEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgEffect::LightSource => write!(f, "light-source"),
            ArgEffect::FishEye => write!(f, "fish-eye"),
            ArgEffect::ColorSource => write!(f, "color-source"),
            ArgEffect::ShockwaveDistortion => write!(f, "shockwave-distortion"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    use std::path::Path;

    #[test]
    fn parse_create_with_defaults() {
        let parsed = AmazeingArgs::try_parse_from(["amazeing", "create", "--rows", "9", "--cols", "11"])
            .expect("create args should parse");

        assert_eq!(parsed.zoom, 1.0);
        assert!(!parsed.show_perimeter);

        match parsed.command {
            ArgCommand::Create(create) => {
                assert_eq!(create.rows, 9);
                assert_eq!(create.cols, 11);
                assert_eq!(create.procedure, ArgProcedure::Dfs);
                assert_eq!(create.heuristic_function, ArgHeuristic::Dijkstra);
                assert!(!create.verbose);
            }
            _ => panic!("expected create command"),
        }
    }

    #[test]
    fn parse_solve_with_global_flags() {
        let parsed = AmazeingArgs::try_parse_from([
            "amazeing",
            "--zoom",
            "1.5",
            "solve",
            "--maze",
            "assets/maze/001_005_005_square.maze",
            "--procedure",
            "a-star",
            "-H",
            "manhattan",
            "--verbose",
            "--fps",
            "24",
        ])
        .expect("solve args should parse");

        assert_eq!(parsed.zoom, 1.5);

        match parsed.command {
            ArgCommand::Solve(solve) => {
                assert_eq!(solve.procedure, ArgProcedure::AStar);
                assert_eq!(solve.heuristic_function, ArgHeuristic::Manhattan);
                assert_eq!(solve.fps, 24.0);
                assert_eq!(solve.maze, Path::new("assets/maze/001_005_005_square.maze"));
            }
            _ => panic!("expected solve command"),
        }
    }

    #[test]
    fn parse_create_with_aldous_broder_procedure() {
        let parsed = AmazeingArgs::try_parse_from([
            "amazeing",
            "create",
            "--rows",
            "9",
            "--cols",
            "11",
            "--procedure",
            "aldous-broder",
        ])
        .expect("create args should parse for aldous-broder");

        match parsed.command {
            ArgCommand::Create(create) => {
                assert_eq!(create.procedure, ArgProcedure::AldousBroder);
            }
            _ => panic!("expected create command"),
        }
    }

    #[test]
    fn parse_solve_with_bidirectional_a_start_procedure() {
        let parsed = AmazeingArgs::try_parse_from([
            "amazeing",
            "solve",
            "--maze",
            "assets/maze/001_005_005_square.maze",
            "--procedure",
            "bidirectional-a-start",
        ])
        .expect("solve args should parse for bidirectional-a-start");

        match parsed.command {
            ArgCommand::Solve(solve) => {
                assert_eq!(solve.procedure, ArgProcedure::BidirectionalAStart);
            }
            _ => panic!("expected solve command"),
        }
    }

    #[test]
    fn parse_create_with_prim_procedure() {
        let parsed =
            AmazeingArgs::try_parse_from(["amazeing", "create", "--rows", "9", "--cols", "11", "--procedure", "prim"])
                .expect("create args should parse for prim");

        match parsed.command {
            ArgCommand::Create(create) => {
                assert_eq!(create.procedure, ArgProcedure::Prim);
            }
            _ => panic!("expected create command"),
        }
    }

    #[test]
    fn parse_solve_with_greedy_best_first_procedure() {
        let parsed = AmazeingArgs::try_parse_from([
            "amazeing",
            "solve",
            "--maze",
            "assets/maze/001_005_005_square.maze",
            "--procedure",
            "greedy-best-first",
        ])
        .expect("solve args should parse for greedy-best-first");

        match parsed.command {
            ArgCommand::Solve(solve) => {
                assert_eq!(solve.procedure, ArgProcedure::GreedyBestFirst);
            }
            _ => panic!("expected solve command"),
        }
    }

    #[test]
    fn parse_create_with_beam_search_procedure() {
        let parsed = AmazeingArgs::try_parse_from([
            "amazeing",
            "create",
            "--rows",
            "9",
            "--cols",
            "11",
            "--procedure",
            "beam-search",
        ])
        .expect("create args should parse for beam-search");

        match parsed.command {
            ArgCommand::Create(create) => assert_eq!(create.procedure, ArgProcedure::BeamSearch),
            _ => panic!("expected create command"),
        }
    }

    #[test]
    fn parse_solve_with_bidirectional_greedy_best_first_procedure() {
        let parsed = AmazeingArgs::try_parse_from([
            "amazeing",
            "solve",
            "--maze",
            "assets/maze/001_005_005_square.maze",
            "--procedure",
            "bidirectional-greedy-best-first",
        ])
        .expect("solve args should parse for bidirectional-greedy-best-first");

        match parsed.command {
            ArgCommand::Solve(solve) => {
                assert_eq!(solve.procedure, ArgProcedure::BidirectionalGreedyBestFirst);
            }
            _ => panic!("expected solve command"),
        }
    }
}
