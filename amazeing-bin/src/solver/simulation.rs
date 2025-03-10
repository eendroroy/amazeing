use crate::command::parse_node::parse_node;
use crate::context::{DrawContext, SOLVER_CONTEXT};
use crate::display::action::quit_requested;
use crate::display::drawer::{
    draw_current_path, draw_maze, draw_path, draw_source_destination, draw_traversed, get_conf,
};
use crate::helper::loader::loader_maze_from_file;
use crate::helper::{current_millis, get_heu};
use amazeing::maze::matrix::Maze;
use amazeing::solver::matrix::{a_star, bfs, dfs, dijkstra};
use amazeing::DNode;
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;

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

        if quit_requested() {
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

#[macroquad::main(get_conf())]
pub async fn main() {
    let maze = SOLVER_CONTEXT.read().unwrap().maze.clone();

    let (margin, padding, cell_width, cell_height) = SOLVER_CONTEXT.read().unwrap().display_size();
    let (maze_width, maze_height) = (maze.cols(), maze.rows());
    let (screen_width, screen_height) = (
        margin + maze_width as f32 * (cell_width + padding) + margin,
        margin + maze_height as f32 * (cell_height + padding) + margin,
    );

    set_window_size(screen_width as u32, screen_height as u32 + 30);

    let mut trace = SOLVER_CONTEXT.read().unwrap().tracer.clone();

    simulation_loop(maze, margin, padding, cell_width, cell_height, &mut trace).await
}

pub(crate) fn simulate(
    algorithm: String,
    heu: String,
    maze_file_path: String,
    from: String,
    to: String,
    fps: String,
    display_size: String,
) {
    SOLVER_CONTEXT.write().unwrap().title = String::from(format!("Maze Solver ({})", algorithm));
    SOLVER_CONTEXT.write().unwrap().display_size = display_size;

    if fps != String::from("") {
        SOLVER_CONTEXT.write().unwrap().fps = u8::from_str_radix(&fps, 10).unwrap();
    }

    let maze = loader_maze_from_file(&*maze_file_path);
    SOLVER_CONTEXT.write().unwrap().maze = maze.clone();

    let mut tracer: Option<Vec<Vec<DNode>>> = Some(vec![]);
    let (source, destination) = (parse_node(&from), parse_node(&to));

    SOLVER_CONTEXT.write().unwrap().source = source;
    SOLVER_CONTEXT.write().unwrap().destination = destination;

    match &*algorithm {
        "bfs" => bfs(&maze, source, destination, &mut tracer),
        "dfs" => dfs(&maze, source, destination, &mut tracer),
        "dijkstra" => dijkstra(&maze, source, destination, &mut tracer),
        "a-star" => a_star(&maze, source, destination, get_heu(&*heu), &mut tracer),
        _ => panic!("Unknown algorithm name {}", algorithm),
    };

    SOLVER_CONTEXT.write().unwrap().tracer = tracer.unwrap();

    main();
}
