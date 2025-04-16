use crate::command::{AmazeingArgs, ArgCommand};
use crate::context::{ColorContext, ColorScheme, CreateContext, DrawContext, SolveContext, ViewContext};
use crate::helper::load_maze_from_file;
use amazeing::matrix::Node;

type GetContextRet = ((Option<CreateContext>, Option<ViewContext>, Option<SolveContext>), DrawContext, ColorContext);
static GRADIENT_STEPS: fn(usize, usize) -> usize = |r, c| ((r + c) as f32 * 0.25).clamp(8., 64.) as usize;

pub(crate) fn get_contexts(args: AmazeingArgs) -> GetContextRet {
    let gradient_steps: usize;

    let amazeing_context = match args.command {
        ArgCommand::Create {
            maze,
            source,
            procedure,
            rows,
            cols,
            ..
        } => {
            gradient_steps = GRADIENT_STEPS(rows, cols);
            (
                Some(CreateContext {
                    maze_file_path: maze.clone(),
                    sources: if let Some(sources) = source { parse_nodes(&sources) } else { Vec::new() },
                    procedure,
                    rows,
                    cols,
                }),
                None,
                None,
            )
        }
        ArgCommand::View { maze, update: _ } => {
            let loaded_maze = load_maze_from_file(maze.as_path());
            gradient_steps = GRADIENT_STEPS(loaded_maze.rows(), loaded_maze.cols());
            (
                None,
                Some(ViewContext {
                    maze_file_path: maze.clone(),
                    maze: loaded_maze,
                }),
                None,
            )
        }
        ArgCommand::Solve {
            maze,
            procedure,
            heuristic_function,
            ..
        } => {
            let loaded_maze = load_maze_from_file(maze.as_path());
            gradient_steps = GRADIENT_STEPS(loaded_maze.rows(), loaded_maze.cols());
            (
                None,
                None,
                Some(SolveContext {
                    maze: loaded_maze,
                    procedure,
                    heuristic: heuristic_function.heuristic(),
                }),
            )
        }
    };

    let draw_ctx = DrawContext::from(args.zoom, args.unit_shape.shape(), args.fps);

    let colors = if let Some(path) = args.colors {
        ColorContext::from(ColorScheme::from(path.as_path()), gradient_steps)
    } else {
        ColorContext::new(gradient_steps)
    };

    (amazeing_context, draw_ctx, colors)
}

fn parse_node(node: &str) -> Node {
    let node_data = node.split(",").collect::<Vec<&str>>();
    (
        node_data.first().unwrap().parse::<u32>().unwrap() as usize,
        node_data.get(1).unwrap().parse::<u32>().unwrap() as usize,
    )
}

fn parse_nodes(nodes: &[String]) -> Vec<Node> {
    nodes.iter().map(|node| parse_node(node)).collect()
}
