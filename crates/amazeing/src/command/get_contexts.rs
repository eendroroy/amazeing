use crate::command::{AmazeingArgs, ArgCommand, ArgMazeShape};
use crate::context::{ColorContext, ColorScheme, CreateContext, DrawContext, SolveContext, ViewContext};
use crate::helper::load_maze_from_file;
use amazeing::tiled::{MazeShape, UnitShape};

type GetContextRet = ((Option<CreateContext>, Option<ViewContext>, Option<SolveContext>), DrawContext, ColorContext);
static GRADIENT_STEPS: fn(usize, usize) -> usize = |r, c| ((r + c) as f32 * 0.25).clamp(8., 64.) as usize;

pub(crate) fn get_contexts(amazeing_args: AmazeingArgs) -> GetContextRet {
    let gradient_steps: usize;
    let unit_shape: UnitShape;
    let maze_shape: MazeShape;

    let mut amz_ctx = match amazeing_args.command {
        ArgCommand::Create(command_args) => {
            let (procedure, rows, cols) = match command_args.maze_shape {
                ArgMazeShape::Triangle(shape_args) => {
                    maze_shape = MazeShape::Triangle;
                    unit_shape = shape_args.unit_shape.as_unit_shape();
                    (shape_args.procedure, shape_args.base, shape_args.base)
                }
                ArgMazeShape::Rectangle(shape_args) => {
                    maze_shape = MazeShape::Rectangle;
                    unit_shape = shape_args.unit_shape.as_unit_shape();
                    (shape_args.procedure, shape_args.rows, shape_args.cols)
                }
                ArgMazeShape::Square(shape_args) => {
                    maze_shape = MazeShape::Square;
                    unit_shape = shape_args.unit_shape.as_unit_shape();
                    (shape_args.procedure, shape_args.size, shape_args.size)
                }
                ArgMazeShape::Circle(shape_args) => {
                    maze_shape = MazeShape::Circle;
                    unit_shape = shape_args.unit_shape.as_unit_shape();
                    (shape_args.procedure, shape_args.diameter, shape_args.diameter)
                }
            };
            gradient_steps = GRADIENT_STEPS(rows, cols);
            (Some(CreateContext::from(command_args.maze.clone(), procedure, rows, cols)), None, None)
        }
        ArgCommand::View(sub_args) => {
            let loaded_maze = load_maze_from_file(sub_args.maze.as_path());
            gradient_steps = GRADIENT_STEPS(loaded_maze.rows(), loaded_maze.cols());
            unit_shape = loaded_maze.unit_shape;
            maze_shape = loaded_maze.maze_shape;
            (None, Some(ViewContext::from(sub_args.maze.clone(), loaded_maze)), None)
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

    let dr_ctx = DrawContext::from(amazeing_args.zoom, maze_shape, unit_shape, amazeing_args.fps);

    if let Some(ref mut c_ctx) = amz_ctx.0 {
        if maze_shape == MazeShape::Triangle {
            if c_ctx.rows % 2 == 0 {
                c_ctx.rows += 1;
                c_ctx.cols += 1;
            }
            if unit_shape == UnitShape::Triangle {
                c_ctx.rows *= 2;
            }
        }
        if maze_shape == MazeShape::Circle {
            if [UnitShape::Circle, UnitShape::Hexagon].contains(&unit_shape) {
                c_ctx.cols = (c_ctx.rows as f32 * dr_ctx.u_height / dr_ctx.u_width) as usize;
            }
            if unit_shape == UnitShape::Triangle {
                c_ctx.rows = (c_ctx.cols as f32 * dr_ctx.u_width / dr_ctx.u_height) as usize * 2;
            }
        }
    }

    let colors = if let Some(path) = amazeing_args.colors {
        ColorContext::from(ColorScheme::from(path.as_path()), gradient_steps)
    } else {
        ColorContext::new(gradient_steps)
    };

    (amz_ctx, dr_ctx, colors)
}
