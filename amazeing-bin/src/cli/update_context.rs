use crate::cli::{AmazeingArgs, ArgDisplay, ArgHeuristic, ArgMode};
use crate::context::{
    ColorContext, ColorScheme, COL_CTX, DRAW_CTX, GEN_CTX, MOD_CTX, REL_CTX, SIM_CTX, VIS_CTX,
};
use crate::helper::loader::loader_maze_from_file;
use amazeing::maze::matrix::{
    chebyshev_heuristic, dijkstra_heuristic, euclidean_heuristic, manhattan_heuristic,
    octile_heuristic,
};
use amazeing::{DNode, HeuFn};

pub(crate) fn update_context(args: AmazeingArgs) {
    match args.mode {
        ArgMode::Generate {
            maze,
            procedure,
            rows,
            cols,
        } => {
            let mut context = GEN_CTX.write().unwrap();
            context.maze_file_path = maze.clone();
            context.procedure = procedure;
            context.rows = rows;
            context.cols = cols;
            DRAW_CTX.write().unwrap().maze_rows = rows;
            DRAW_CTX.write().unwrap().maze_cols = cols;
        }
        ArgMode::Visualize { maze } => {
            let mut context = VIS_CTX.write().unwrap();
            context.maze_file_path = maze.clone();
            context.maze = loader_maze_from_file(maze.as_path());
            DRAW_CTX.write().unwrap().maze_rows = context.maze.rows();
            DRAW_CTX.write().unwrap().maze_cols = context.maze.cols();
        }
        ArgMode::Modify { maze } => {
            let mut context = MOD_CTX.write().unwrap();
            context.maze_file_path = maze.clone();
            context.maze = loader_maze_from_file(maze.as_path());
            DRAW_CTX.write().unwrap().maze_rows = context.maze.rows();
            DRAW_CTX.write().unwrap().maze_cols = context.maze.cols();
        }
        ArgMode::Simulate {
            maze,
            source,
            destination,
            procedure,
            heuristic_function,
            fps,
        } => {
            let mut context = SIM_CTX.write().unwrap();
            context.maze_file_path = maze.clone();
            context.maze = loader_maze_from_file(maze.as_path());
            context.proc = procedure;
            context.source = parse_node(&source);
            context.destination = parse_node(&destination);
            if let Some(value) = heuristic_function {
                context.heuristic = get_heu_fn(value)
            }
            if let Some(value) = fps {
                context.fps = value
            }
            DRAW_CTX.write().unwrap().maze_rows = context.maze.rows();
            DRAW_CTX.write().unwrap().maze_cols = context.maze.cols();
        }
        ArgMode::Realtime {
            maze,
            procedure,
            heuristic_function,
        } => {
            let mut context = REL_CTX.write().unwrap();
            context.maze_file_path = maze.clone();
            context.maze = loader_maze_from_file(maze.as_path());
            context.proc = procedure;
            if let Some(value) = heuristic_function {
                context.heuristic = get_heu_fn(value)
            }
            DRAW_CTX.write().unwrap().maze_rows = context.maze.rows();
            DRAW_CTX.write().unwrap().maze_cols = context.maze.cols();
        }
    }

    let mut ctx = DRAW_CTX.write().unwrap();

    match args.display_size {
        Some(ArgDisplay::XXS) => ctx.set((3., 1., 3., 3.)),
        Some(ArgDisplay::XS) => ctx.set((5., 1., 5., 5.)),
        Some(ArgDisplay::S) => ctx.set((10., 1., 10., 10.)),
        Some(ArgDisplay::M) => ctx.set((15., 2., 15., 15.)),
        Some(ArgDisplay::L) => ctx.set((25., 3., 20., 20.)),
        Some(ArgDisplay::XL) => ctx.set((30., 4., 30., 30.)),
        Some(ArgDisplay::XXL) => ctx.set((40., 6., 40., 40.)),
        None => {}
    }

    if let Some(scale) = args.display_scale {
        ctx.scale(scale);
    }

    if let Some(path) = args.color_scheme {
        let colors = ColorContext::from(ColorScheme::from(path.as_path()));
        *COL_CTX.write().unwrap() = colors;
    }
}

fn parse_node(node: &str) -> DNode {
    let node_data = node.split(",").collect::<Vec<&str>>();
    (
        u32::from_str_radix(node_data.get(0).unwrap(), 10).unwrap() as usize,
        u32::from_str_radix(node_data.get(1).unwrap(), 10).unwrap() as usize,
    )
}

fn get_heu_fn(value: ArgHeuristic) -> HeuFn {
    match value {
        ArgHeuristic::Manhattan => manhattan_heuristic,
        ArgHeuristic::Euclidean => euclidean_heuristic,
        ArgHeuristic::Chebyshev => chebyshev_heuristic,
        ArgHeuristic::Octile => octile_heuristic,
        ArgHeuristic::Dijkstra => dijkstra_heuristic,
    }
}
