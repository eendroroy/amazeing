use colored::Colorize;

fn header(text: &str) -> String {
    format!("{}", text.truecolor(162, 190, 140).bold())
}

fn command(text: &str) -> String {
    format!("{}", text.truecolor(143, 188, 187).bold())
}

fn value(text: &str) -> String {
    format!("{}", text.truecolor(135, 192, 208))
}

fn description(text: &str) -> String {
    format!("{}", text.truecolor(216, 222, 233))
}

pub(crate) fn help() {
    println!("{} {}", header("Usage:"), command("amazeing"));
    println!();
    println!("{}", header("Options:"));
    println!(
        "{} {}",
        command("    -h, --help"),
        description("                Print the help menu")
    );
    println!(
        "{} {} {}",
        command("        --path"),
        value("    str"),
        description("        Path to the maze file"),
    );
    println!(
        "{} {} {}",
        command("        --from"),
        value("    usize,usize"),
        description("Start point"),
    );
    println!(
        "{} {} {}",
        command("        --to"),
        value("      usize,usize"),
        description("End point"),
    );
    println!(
        "{} {}",
        command("        --bfs"),
        description("                 Run the simulation for BFS")
    );
    println!(
        "{} {}",
        command("        --dfs"),
        description("                 Run the simulation for DFS")
    );
    println!(
        "{} {}",
        command("        --dijkstra"),
        description("            Run the simulation for Dijkstra")
    );
    println!(
        "{} {}",
        command("        --a-star"),
        description("              Run the simulation for A*")
    );
    println!(
        "{} {} {} {} {} {}",
        command("        --heu"),
        value("     str"),
        description("        Heuristic function to use with A*"),
        description("\n                               Choose from:"),
        description("manhattan, euclidean, chebyshev, octile, dijkstra"),
        description("\n                               Default dijkstra if none provided"),
    );
    println!(
        "{} {} {}",
        command("        --fps"),
        value("     u8"),
        description("         Gui FPS"),
    );
    std::process::exit(0);
}
