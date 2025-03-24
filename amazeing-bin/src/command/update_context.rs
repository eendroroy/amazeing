use crate::command::{AmazeingArgs, ArgDisplaySize, ArgHeuristic, ArgMode};
use crate::context::{ColorContext, ColorScheme, COLOR_CTX, DRAW_CTX, GEN_CTX, SOLVE_CTX, VIS_CTX};
use crate::helper::load_maze_from_file;
use amazeing::matrix::heuristics::{
    chebyshev_heuristic, dijkstra_heuristic, euclidean_heuristic, manhattan_heuristic,
    octile_heuristic,
};
use amazeing::matrix::{Node, NodeHeuFn};

pub(crate) fn update_context(args: AmazeingArgs) {
    match args.mode {
        ArgMode::Create {
            maze,
            source,
            procedure,
            rows,
            cols,
            simulate: _,
            fps,
        } => {
            let mut context = GEN_CTX.write().unwrap();
            context.maze_file_path = maze.clone();
            context.source = parse_node(&source);
            context.procedure = procedure;
            context.rows = rows;
            context.cols = cols;
            context.fps = fps;
            DRAW_CTX.write().unwrap().maze_rows = rows;
            DRAW_CTX.write().unwrap().maze_cols = cols;
        }
        ArgMode::View { maze, update: _ } => {
            let mut context = VIS_CTX.write().unwrap();
            context.maze_file_path = maze.clone();
            context.maze = load_maze_from_file(maze.as_path());
            DRAW_CTX.write().unwrap().maze_rows = context.maze.rows();
            DRAW_CTX.write().unwrap().maze_cols = context.maze.cols();
        }
        ArgMode::Solve {
            simulate: _,
            maze,
            procedure,
            heuristic_function,
            fps,
        } => {
            let mut context = SOLVE_CTX.write().unwrap();
            context.maze_file_path = maze.clone();
            context.maze = load_maze_from_file(maze.as_path());
            context.proc = procedure;
            if let Some(value) = heuristic_function {
                context.heuristic = get_heu_fn(value)
            }
            context.fps = fps;
            DRAW_CTX.write().unwrap().maze_rows = context.maze.rows();
            DRAW_CTX.write().unwrap().maze_cols = context.maze.cols();
        }
    }

    let mut ctx = DRAW_CTX.write().unwrap();

    match args.display_size {
        Some(ArgDisplaySize::XXS) => ctx.size((3., 1., 3., 3.)),
        Some(ArgDisplaySize::XS) => ctx.size((5., 1., 5., 5.)),
        Some(ArgDisplaySize::S) => ctx.size((10., 2., 10., 10.)),
        Some(ArgDisplaySize::M) => ctx.size((15., 3., 15., 15.)),
        Some(ArgDisplaySize::L) => ctx.size((25., 4., 20., 20.)),
        Some(ArgDisplaySize::XL) => ctx.size((30., 5., 30., 30.)),
        Some(ArgDisplaySize::XXL) => ctx.size((40., 6., 40., 40.)),
        None => {}
    }

    if let Some(density) = args.display_density {
        ctx.density(density);
    }

    if let Some(path) = args.color_scheme {
        let colors = ColorContext::from(ColorScheme::from(path.as_path()));
        *COLOR_CTX.write().unwrap() = colors;
    }
}

fn get_heu_fn(value: ArgHeuristic) -> NodeHeuFn {
    match value {
        ArgHeuristic::Manhattan => manhattan_heuristic,
        ArgHeuristic::Euclidean => euclidean_heuristic,
        ArgHeuristic::Chebyshev => chebyshev_heuristic,
        ArgHeuristic::Octile => octile_heuristic,
        ArgHeuristic::Dijkstra => dijkstra_heuristic,
    }
}

fn parse_node(node: &str) -> Node {
    let node_data = node.split(",").collect::<Vec<&str>>();
    (
        u32::from_str_radix(node_data.get(0).unwrap(), 10).unwrap() as usize,
        u32::from_str_radix(node_data.get(1).unwrap(), 10).unwrap() as usize,
    )
}
