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
        "{} {} {} {}",
        command("        --algorithm"),
        description("           Algorithm name for simulation"),
        description("\n                               Choose from:"),
        description("bfs, dfs, dijkstra, a-star"),
    );
    println!(
        "{} {} {} {} {} {}",
        command("        --heu"),
        value("     str"),
        description("        Heuristic function to use with a-star"),
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
