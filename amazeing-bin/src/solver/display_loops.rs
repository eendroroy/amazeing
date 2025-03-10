use crate::context::{DrawContext, COLORS, SOLVER_CONTEXT};
use amazeing::maze::matrix::Maze;
use amazeing::solver::matrix::{a_star, bfs, dfs, dijkstra};
use amazeing::DNode;
use macroquad::input::{
    is_key_down, is_key_pressed, is_mouse_button_pressed, mouse_position, KeyCode, MouseButton,
};
use macroquad::prelude::{clear_background, next_frame};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::display::drawer::{draw_current_path, draw_maze, draw_path, draw_source_destination, draw_traversed};

fn current_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

pub(crate) async fn simulation_loop(
    maze: Maze,
    margin: f32,
    padding: f32,
    cell_width: f32,
    cell_height: f32,
    trace: &mut Vec<Vec<DNode>>,
) {
    let ctx = &DrawContext {
        margin,
        padding,
        cell_width,
        cell_height,
    };
    let mut current_path: Vec<DNode> = vec![];
    let mut last_millis = current_millis();
    let update_interval = 1000 / SOLVER_CONTEXT.read().unwrap().fps as u128;

    let mut traversed: Maze = Maze::from(vec![vec![0u32; maze.cols()]; maze.rows()]);

    let mut trace_complete = false;
    let mut simulating = false;

    loop {
        if is_key_pressed(KeyCode::S) && !simulating {
            println!("Starting Simulation");
            simulating = true;
            current_path = trace.remove(0);
            last_millis = current_millis();
            if trace.len() == 0 {
                trace_complete = true;
            }
        }

        if is_key_pressed(KeyCode::Q) {
            println!("Quitting");
            break;
        }

        if simulating {
            if !trace_complete && last_millis + update_interval <= current_millis() {
                current_path = trace.remove(0);
                last_millis = current_millis();
                if trace.len() == 0 {
                    trace_complete = true;
                }
            }

            draw_maze(&maze, ctx);
            draw_traversed(&traversed, ctx);
            if trace_complete {
                draw_path(current_path.clone(), ctx);
            } else {
                draw_current_path(current_path.clone(), ctx, &mut traversed);
            };
        } else {
            draw_maze(&maze, ctx);
        }
        draw_source_destination(
            SOLVER_CONTEXT.read().unwrap().source,
            SOLVER_CONTEXT.read().unwrap().destination,
            ctx,
        );
        next_frame().await
    }
}

pub(crate) async fn realtime_loop(
    maze: Maze,
    margin: f32,
    padding: f32,
    cell_width: f32,
    cell_height: f32,
) {
    let ctx = &DrawContext {
        margin,
        padding,
        cell_width,
        cell_height,
    };
    let mut current_path: Vec<DNode> = vec![];
    let mut from: Option<DNode> = None;
    let mut to: Option<DNode> = None;

    loop {
        clear_background(COLORS.color_bg);
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let r = ((my - margin) / (cell_height + padding)).floor();
            let c = ((mx - margin) / (cell_width + padding)).floor();

            if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                to = Some((r as usize, c as usize));
                SOLVER_CONTEXT.write().unwrap().destination = to.unwrap();
            } else {
                from = Some((r as usize, c as usize));
                SOLVER_CONTEXT.write().unwrap().source = from.unwrap();
            };

            if from.is_some() && to.is_some() {
                current_path = match &*SOLVER_CONTEXT.read().unwrap().algorithm {
                    "bfs" => bfs(&maze, from.unwrap(), to.unwrap(), &mut None),
                    "dfs" => dfs(&maze, from.unwrap(), to.unwrap(), &mut None),
                    "dijkstra" => dijkstra(&maze, from.unwrap(), to.unwrap(), &mut None),
                    "a-star" => a_star(
                        &maze,
                        from.unwrap(),
                        to.unwrap(),
                        crate::solver::solve::get_heu(&*SOLVER_CONTEXT.read().unwrap().heu),
                        &mut None,
                    ),
                    name => panic!("Unknown algorithm name {}", name),
                };
            }
        }

        if is_key_pressed(KeyCode::Q) {
            println!("Quitting");
            break;
        }

        draw_maze(&maze, ctx);
        draw_path(current_path.clone(), ctx);
        draw_source_destination(
            SOLVER_CONTEXT.read().unwrap().source,
            SOLVER_CONTEXT.read().unwrap().destination,
            ctx,
        );
        next_frame().await
    }
}
