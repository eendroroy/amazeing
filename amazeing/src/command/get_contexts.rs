use crate::command::{AmazeingArgs, ArgCommand, ArgMazeShape};
use crate::context::{ColorContext, ColorScheme, CreateContext, DrawContext, SolveContext, ViewContext};
use crate::helper::load_maze_from_file;
use amazeing::tiled::{MazeShape, UnitShape};

type GetContextRet = ((Option<CreateContext>, Option<ViewContext>, Option<SolveContext>), DrawContext, ColorContext);
static GRADIENT_STEPS: fn(usize, usize) -> usize = |r, c| ((r + c) as f32 * 0.25).clamp(8., 64.) as usize;

pub(crate) fn get_contexts(args: AmazeingArgs) -> GetContextRet {
    let gradient_steps: usize;
    let unit_shape: UnitShape;
    let maze_shape: MazeShape;

    let amazeing_context = match args.command {
        ArgCommand::Create(sub_args) => match sub_args.maze_shape {
            ArgMazeShape::Rectangle(rectangle_args) => {
                gradient_steps = GRADIENT_STEPS(rectangle_args.rows, rectangle_args.cols);
                maze_shape = MazeShape::Rectangle;
                unit_shape = rectangle_args.unit_shape.as_unit_shape();
                (
                    Some(CreateContext {
                        maze_file_path: sub_args.maze.clone(),
                        procedure: rectangle_args.procedure,
                        rows: rectangle_args.rows,
                        cols: rectangle_args.cols,
                    }),
                    None,
                    None,
                )
            }
            ArgMazeShape::Square(square_args) => {
                gradient_steps = GRADIENT_STEPS(square_args.size, square_args.size);
                maze_shape = MazeShape::Square;
                unit_shape = square_args.unit_shape.as_unit_shape();
                (
                    Some(CreateContext {
                        maze_file_path: sub_args.maze.clone(),
                        procedure: square_args.procedure,
                        rows: square_args.size,
                        cols: square_args.size,
                    }),
                    None,
                    None,
                )
            }
        },
        ArgCommand::View(sub_args) => {
            let loaded_maze = load_maze_from_file(sub_args.maze.as_path());
            gradient_steps = GRADIENT_STEPS(loaded_maze.rows(), loaded_maze.cols());
            unit_shape = loaded_maze.unit_shape;
            maze_shape = loaded_maze.maze_shape;
            (
                None,
                Some(ViewContext {
                    maze_file_path: sub_args.maze.clone(),
                    maze: loaded_maze,
                }),
                None,
            )
        }
        ArgCommand::Solve(sub_args) => {
            let loaded_maze = load_maze_from_file(sub_args.maze.as_path());
            gradient_steps = GRADIENT_STEPS(loaded_maze.rows(), loaded_maze.cols());
            unit_shape = loaded_maze.unit_shape;
            maze_shape = loaded_maze.maze_shape;
            (
                None,
                None,
                Some(SolveContext {
                    maze: loaded_maze,
                    procedure: sub_args.procedure,
                    heuristic: sub_args.heuristic_function.as_node_heu_fn(),
                }),
            )
        }
    };

    let draw_ctx = DrawContext::from(args.zoom, maze_shape, unit_shape, args.fps);

    let colors = if let Some(path) = args.colors {
        ColorContext::from(ColorScheme::from(path.as_path()), gradient_steps)
    } else {
        ColorContext::new(gradient_steps)
    };

    (amazeing_context, draw_ctx, colors)
}
