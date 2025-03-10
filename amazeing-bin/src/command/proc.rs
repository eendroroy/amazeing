#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Proc {
    None,
    Dfs,
    Bfs,
    Dijkstra,
    AStar,
}

pub fn get_proc(str: &str) -> Proc {
    match str {
        "dfs" => Proc::Dfs,
        "bfs" => Proc::Bfs,
        "dijkstra" => Proc::Dijkstra,
        "a-star" => Proc::AStar,
        _ => Proc::None,
    }
}
