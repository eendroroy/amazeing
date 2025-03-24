use crate::command::{AmazeingArgs, ArgDisplaySize, ArgHeuristic, ArgMode};
use crate::context::{
    ColorContext, ColorScheme, CreateContext, DrawContext, SolveContext, ViewContext,
};
use crate::helper::load_maze_from_file;
use amazeing::matrix::heuristics::{
    chebyshev_heuristic, dijkstra_heuristic, euclidean_heuristic, manhattan_heuristic,
    octile_heuristic,
};
use amazeing::matrix::{Node, NodeHeuFn};

type GetContextRet = (
    (
        Option<CreateContext>,
        Option<ViewContext>,
        Option<SolveContext>,
    ),
    DrawContext,
    ColorContext,
);

pub(crate) fn get_contexts(args: AmazeingArgs) -> GetContextRet {
    let mut draw_ctx = DrawContext::new();

    let amazeing_context = match args.mode {
        ArgMode::Create {
            maze,
            source,
            procedure,
            rows,
            cols,
            fps,
            ..
        } => {
            draw_ctx.maze_rows = rows;
            draw_ctx.maze_cols = cols;
            (
                Some(CreateContext {
                    maze_file_path: maze.clone(),
                    source: parse_node(&source),
                    procedure,
                    rows,
                    cols,
                    fps,
                }),
                None,
                None,
            )
        }
        ArgMode::View { maze, update: _ } => {
            let loaded_maze = load_maze_from_file(maze.as_path());
            draw_ctx.maze_rows = loaded_maze.rows();
            draw_ctx.maze_cols = loaded_maze.cols();
            (
                None,
                Some(ViewContext {
                    maze_file_path: maze.clone(),
                    maze: loaded_maze,
                }),
                None,
            )
        }
        ArgMode::Solve {
            maze,
            procedure,
            heuristic_function,
            fps,
            ..
        } => {
            let loaded_maze = load_maze_from_file(maze.as_path());
            draw_ctx.maze_rows = loaded_maze.rows();
            draw_ctx.maze_cols = loaded_maze.cols();
            (
                None,
                None,
                Some(SolveContext {
                    maze: loaded_maze,
                    procedure,
                    fps,
                    heuristic: if let Some(value) = heuristic_function {
                        get_heu_fn(value)
                    } else {
                        dijkstra_heuristic
                    },
                }),
            )
        }
    };

    match args.display_size {
        Some(ArgDisplaySize::XXS) => draw_ctx.size((3., 1., 3., 3.)),
        Some(ArgDisplaySize::XS) => draw_ctx.size((5., 1., 5., 5.)),
        Some(ArgDisplaySize::S) => draw_ctx.size((10., 2., 10., 10.)),
        Some(ArgDisplaySize::M) => draw_ctx.size((15., 3., 15., 15.)),
        Some(ArgDisplaySize::L) => draw_ctx.size((25., 4., 20., 20.)),
        Some(ArgDisplaySize::XL) => draw_ctx.size((30., 5., 30., 30.)),
        Some(ArgDisplaySize::XXL) => draw_ctx.size((40., 6., 40., 40.)),
        None => {}
    }

    if let Some(density) = args.display_density {
        draw_ctx.density(density);
    }

    let colors = if let Some(path) = args.color_scheme {
        ColorContext::from(ColorScheme::from(path.as_path()))
    } else {
        ColorContext::new()
    };

    (amazeing_context, draw_ctx, colors)
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
