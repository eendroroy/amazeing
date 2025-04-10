use crate::command::{AmazeingArgs, ArgCommand, ArgDisplayDensity, ArgDisplaySize, ArgHeuristic, ArgShape};
use crate::context::{ColorContext, ColorScheme, CreateContext, DrawContext, SolveContext, ViewContext};
use crate::helper::load_maze_from_file;
use amazeing::matrix::Node;

type GetContextRet = ((Option<CreateContext>, Option<ViewContext>, Option<SolveContext>), DrawContext, ColorContext);

pub(crate) fn get_contexts(args: AmazeingArgs) -> GetContextRet {
    let amazeing_context = match args.command {
        ArgCommand::Create {
            maze,
            source,
            procedure,
            rows,
            cols,
            tempo,
            ..
        } => (
            Some(CreateContext {
                maze_file_path: maze.clone(),
                sources: if let Some(sources) = source { parse_nodes(&sources) } else { Vec::new() },
                procedure,
                rows,
                cols,
                tempo,
            }),
            None,
            None,
        ),
        ArgCommand::View { maze, update: _ } => {
            let loaded_maze = load_maze_from_file(maze.as_path());
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
            tempo,
            ..
        } => {
            let loaded_maze = load_maze_from_file(maze.as_path());
            (
                None,
                None,
                Some(SolveContext {
                    maze: loaded_maze,
                    procedure,
                    tempo,
                    heuristic: heuristic_function.unwrap_or(ArgHeuristic::Dijkstra).heuristic(),
                }),
            )
        }
    };

    let draw_ctx = DrawContext::from(
        args.display_density.unwrap_or(ArgDisplayDensity::Standard),
        args.display_size.unwrap_or(ArgDisplaySize::M),
        args.shape.unwrap_or(ArgShape::Square).shape(),
    );

    let colors = if let Some(path) = args.color_scheme {
        ColorContext::from(ColorScheme::from(path.as_path()))
    } else {
        ColorContext::default()
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
