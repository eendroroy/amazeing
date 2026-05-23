use crate::app::common::{get_colors, set_screen_size};
use crate::cli::{AmazeingArgs, AmazeingContext, ArgEffect, CreateArgs};
use crate::render::display_loop::{generate_loop, generate_simulation_loop};
use crate::render::scene::MazeScene;

pub(super) async fn run(global: &AmazeingArgs, args: CreateArgs) {
    let light_source_effect = global.effect.contains(&ArgEffect::LightSource);
    let fisheye_effect = global.effect.contains(&ArgEffect::FishEye);
    let context = AmazeingContext::create_context(
        None,
        args.maze,
        args.procedure,
        args.heuristic_function.heuristic(),
        args.jumble_factor,
        args.weight_direction.direction(),
        args.rows,
        args.cols,
        global.zoom,
        global.fps,
        global.show_perimeter,
        light_source_effect,
        fisheye_effect,
    );

    let mut scene = MazeScene::new_from_dimension(
        args.unit_shape.shape(),
        &context,
        &get_colors(context.rows, context.cols, global.colors.as_ref()),
    );

    set_screen_size(scene.wh);
    if args.verbose {
        generate_simulation_loop(&mut scene).await
    } else {
        generate_loop(&mut scene).await
    }
}
