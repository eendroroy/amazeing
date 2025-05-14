use crate::command::{AmazeingArgs, ArgCommand, ArgMazeShape};
use crate::core::tiled::{Maze, MazeShape, UnitShape, BLOCK};
use crate::ui::context::{AmazeingContext, AmazingCommandType, ColorScheme, Colors, DrawContext};
use crate::ui::helper::load_maze_from_file;

static GRADIENT_STEPS: fn(usize, usize) -> usize = |r, c| ((r + c) as f32 * 0.25).clamp(8., 64.) as usize;

pub(crate) fn get_contexts(amazeing_args: AmazeingArgs) -> (AmazeingContext, DrawContext, Colors) {
    let gradient_steps: usize;
    let unit_shape: UnitShape;
    let maze_shape: MazeShape;

    let mut amz_ctx = match amazeing_args.command {
        ArgCommand::Create(command_args) => {
            let (rows, cols) = match command_args.maze_shape {
                ArgMazeShape::Triangle(shape_args) => {
                    maze_shape = MazeShape::Triangle;
                    unit_shape = shape_args.unit_shape.as_unit_shape();
                    (shape_args.base, shape_args.base)
                }
                ArgMazeShape::Rectangle(shape_args) => {
                    maze_shape = MazeShape::Rectangle;
                    unit_shape = shape_args.unit_shape.as_unit_shape();
                    (shape_args.rows, shape_args.cols)
                }
                ArgMazeShape::Circle(shape_args) => {
                    maze_shape = MazeShape::Circle;
                    unit_shape = shape_args.unit_shape.as_unit_shape();
                    (shape_args.diameter, shape_args.diameter)
                }
                ArgMazeShape::Hexagon(shape_args) => {
                    maze_shape = MazeShape::Hexagon;
                    unit_shape = shape_args.unit_shape.as_unit_shape();
                    (shape_args.diameter, shape_args.diameter)
                }
            };
            gradient_steps = GRADIENT_STEPS(rows, cols);

            AmazeingContext::create_context(
                Maze::from(maze_shape, unit_shape, vec![vec![BLOCK; cols]; rows]),
                command_args.maze,
                command_args.procedure,
                command_args.heuristic_function.as_node_heu_fn(),
                command_args.jumble_factor,
                command_args.weight_direction.as_weight_direction(),
                (rows, cols),
            )
        }
        ArgCommand::View(sub_args) => {
            let loaded_maze = load_maze_from_file(sub_args.maze.as_path());
            gradient_steps = GRADIENT_STEPS(loaded_maze.rows(), loaded_maze.cols());
            unit_shape = loaded_maze.unit_shape;
            maze_shape = loaded_maze.maze_shape;
            AmazeingContext::view_context(loaded_maze, sub_args.maze.clone())
        }
        ArgCommand::Solve(sub_args) => {
            let loaded_maze = load_maze_from_file(sub_args.maze.as_path());
            gradient_steps = GRADIENT_STEPS(loaded_maze.rows(), loaded_maze.cols());
            unit_shape = loaded_maze.unit_shape;
            maze_shape = loaded_maze.maze_shape;
            AmazeingContext::solve_context(
                loaded_maze,
                sub_args.procedure,
                sub_args.heuristic_function.as_node_heu_fn(),
            )
        }
    };

    let dr_ctx = DrawContext::from(amazeing_args.zoom, maze_shape, unit_shape, amazeing_args.fps);

    if amz_ctx.command_type == AmazingCommandType::Create {
        if [MazeShape::Triangle, MazeShape::Circle].contains(&maze_shape) {
            if amz_ctx.rows % 2 == 0 {
                amz_ctx.rows += 1;
                amz_ctx.cols += 1;
            }
            if unit_shape == UnitShape::Triangle {
                amz_ctx.rows *= 2;
            }
        }
        if maze_shape == MazeShape::Circle {
            match unit_shape {
                UnitShape::Hexagon => {
                    amz_ctx.cols = (amz_ctx.rows as f32 * dr_ctx.unit_height / dr_ctx.unit_width) as usize;
                }
                UnitShape::Triangle => {
                    amz_ctx.rows = (amz_ctx.cols as f32 * dr_ctx.unit_width / dr_ctx.unit_height) as usize * 2;
                }
                _ => {}
            }
        }
    }

    let colors = if let Some(path) = amazeing_args.colors {
        Colors::from(ColorScheme::from(path.as_path()), gradient_steps)
    } else {
        Colors::new(gradient_steps)
    };

    (amz_ctx, dr_ctx, colors)
}
