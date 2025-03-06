use crate::help;
use std::env;

pub(crate) fn parse_arg() -> (String, String, String, String, String, String) {
    let mut args = env::args().skip(1);

    let mut algorithm = "";
    let mut heu = String::from("");
    let mut path = String::from("");
    let mut from = String::from("");
    let mut to = String::from("");
    let mut fps = String::from("");

    while let Some(arg) = args.next() {
        match &arg[..] {
            "-h" | "--help" => help::help(),
            "--bfs" => algorithm = "bfs",
            "--dfs" => algorithm = "dfs",
            "--dijkstra" => algorithm = "dijkstra",
            "--a-star" => algorithm = "a-star",
            "--heu" => heu = args.next().unwrap(),
            "--path" => path = args.next().unwrap(),
            "--from" => from = args.next().unwrap(),
            "--to" => to = args.next().unwrap(),
            "--fps" => fps = args.next().unwrap(),
            _ => {
                if arg.starts_with('-') {
                    println!("Unknown argument {}", arg);
                } else {
                    println!("Unknown positional argument {}", arg);
                }
            }
        }
    }

    (String::from(algorithm), heu, path, from, to, fps)
}
