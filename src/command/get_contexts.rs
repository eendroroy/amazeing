use crate::command::{AmazeingArgs, ArgCommand, ArgMazeShape};
use crate::ui::context::{AmazeingContext, ColorScheme, Colors};
use crate::ui::helper::load_maze_from_file;

macro_rules! gradient_steps {
    ($r:expr, $c:expr) => {
        ((($r + $c) as f32 * 0.25).clamp(8., 64.) as usize)
    };
}

pub(crate) fn get_contexts(amazeing_args: AmazeingArgs) -> (AmazeingContext, Colors) {
    let context = match amazeing_args.command {
        ArgCommand::Create(command_args) => {
            let (rows, cols) = match command_args.maze_shape {
                ArgMazeShape::Triangle(shape_args) => (shape_args.base, shape_args.base),
                ArgMazeShape::Rectangle(shape_args) => (shape_args.rows, shape_args.cols),
                ArgMazeShape::Circle(shape_args) | ArgMazeShape::Hexagon(shape_args) => {
                    (shape_args.diameter, shape_args.diameter)
                }
            };

            AmazeingContext::create_context(
                (None, command_args.maze),
                (command_args.procedure, command_args.heuristic_function.heuristic()),
                (command_args.jumble_factor, command_args.weight_direction.direction()),
                (rows, cols),
                (amazeing_args.zoom, amazeing_args.fps, amazeing_args.show_perimeter),
            )
        }
        ArgCommand::View(sub_args) => AmazeingContext::view_context(
            (load_maze_from_file(sub_args.maze.as_path()), sub_args.maze.clone()),
            (amazeing_args.zoom, amazeing_args.fps, amazeing_args.show_perimeter),
        ),
        ArgCommand::Solve(sub_args) => AmazeingContext::solve_context(
            load_maze_from_file(sub_args.maze.as_path()),
            (sub_args.procedure, sub_args.heuristic_function.heuristic()),
            (amazeing_args.zoom, amazeing_args.fps, amazeing_args.show_perimeter),
        ),
    };

    let gradient_steps: usize = gradient_steps![context.rows, context.cols];
    let colors = if let Some(path) = amazeing_args.colors {
        Colors::from(ColorScheme::from(path.as_path()), gradient_steps)
    } else {
        Colors::new(gradient_steps)
    };

    (context, colors)
}
