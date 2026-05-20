use crate::app::common::{get_colors, set_screen_size};
use crate::cli::{AmazeingArgs, AmazeingContext, ViewArgs};
use crate::render::display_loop::{update_loop, view_loop};
use crate::render::helper::load_maze_from_file;
use crate::render::scene::MazeScene;

pub(super) async fn run(global: &AmazeingArgs, args: ViewArgs) {
    let maze = load_maze_from_file(args.maze.as_path());
    let context = AmazeingContext::view_context(maze, args.maze.clone(), global.zoom, global.fps, global.show_perimeter);

    let mut scene = MazeScene::new_from_maze(
        context.maze.as_ref().expect("view context always includes maze"),
        &context,
        &get_colors(context.rows, context.cols, global.colors.as_ref()),
    );

    set_screen_size(scene.wh);
    if args.update {
        update_loop(&mut scene).await
    } else {
        view_loop(scene).await
    }
}
