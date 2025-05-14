use crate::command::{AmazeingArgs, ArgCommand, ArgMazeShape};
use crate::ui::context::{AmazeingContext, ColorScheme, Colors};
use crate::ui::helper::load_maze_from_file;

static GRADIENT_STEPS: fn(usize, usize) -> usize = |r, c| ((r + c) as f32 * 0.25).clamp(8., 64.) as usize;

pub(crate) fn get_contexts(amazeing_args: AmazeingArgs) -> (AmazeingContext, Colors) {
    let gradient_steps: usize;

    let amz_ctx = match amazeing_args.command {
        ArgCommand::Create(command_args) => {
            let (rows, cols) = match command_args.maze_shape {
                ArgMazeShape::Triangle(shape_args) => (shape_args.base, shape_args.base),
                ArgMazeShape::Rectangle(shape_args) => (shape_args.rows, shape_args.cols),
                ArgMazeShape::Circle(shape_args) | ArgMazeShape::Hexagon(shape_args) => {
                    (shape_args.diameter, shape_args.diameter)
                }
            };
            gradient_steps = GRADIENT_STEPS(rows, cols);

            AmazeingContext::create_context(
                None,
                command_args.maze,
                command_args.procedure,
                command_args.heuristic_function.heuristic(),
                command_args.jumble_factor,
                command_args.weight_direction.direction(),
                (rows, cols),
            )
        }
        ArgCommand::View(sub_args) => {
            let loaded_maze = load_maze_from_file(sub_args.maze.as_path());
            gradient_steps = GRADIENT_STEPS(loaded_maze.rows(), loaded_maze.cols());
            AmazeingContext::view_context(loaded_maze, sub_args.maze.clone())
        }
        ArgCommand::Solve(sub_args) => {
            let loaded_maze = load_maze_from_file(sub_args.maze.as_path());
            gradient_steps = GRADIENT_STEPS(loaded_maze.rows(), loaded_maze.cols());
            AmazeingContext::solve_context(loaded_maze, sub_args.procedure, sub_args.heuristic_function.heuristic())
        }
    };

    let colors = if let Some(path) = amazeing_args.colors {
        Colors::from(ColorScheme::from(path.as_path()), gradient_steps)
    } else {
        Colors::new(gradient_steps)
    };

    (amz_ctx, colors)
}
