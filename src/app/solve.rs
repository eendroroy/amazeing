use crate::app::common::{get_colors, set_screen_size};
use crate::cli::{AmazeingArgs, AmazeingContext, EffectOptions, SolveArgs};
use crate::render::display_loop::{solve_loop, solve_simulation_loop};
use crate::render::helper::load_maze_from_file;
use crate::render::scene::MazeScene;

pub(super) async fn run(global: &AmazeingArgs, args: SolveArgs) {
    let maze = load_maze_from_file(args.maze.as_path());
    let context = AmazeingContext::solve_context(
        maze,
        (args.procedure, args.heuristic_function.heuristic()),
        (global.zoom, args.fps, global.show_perimeter),
        EffectOptions::from_args(&args.effect),
    );

    let mut scene = MazeScene::new_from_maze(
        context.maze.as_ref().expect("solve context always includes maze"),
        &context,
        &get_colors(context.rows, context.cols, global.colors.as_ref()),
    );

    set_screen_size(scene.wh);
    if args.verbose {
        solve_simulation_loop(&mut scene).await
    } else {
        solve_loop(&mut scene).await
    }
}
