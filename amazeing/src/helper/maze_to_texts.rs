use amazeing::matrix::{Maze, Node};
use std::collections::HashMap;

pub(crate) fn maze_html_text(
    maze: &Maze,
    traversed: Option<&mut Maze>,
    path: Option<&HashMap<Node, bool>>,
    (sources, destination): (Vec<Node>, Option<Node>),
    traversing: bool,
) -> String {
    let mut tr_list: Vec<String> = vec![];
    for r in 0..maze.rows() {
        let mut td_list: Vec<String> = vec![];
        for c in 0..maze.cols() {
            let node = (r, c);
            let is_traversed = if let Some(ref t) = traversed { t[node] == 1 } else { false };
            let class = if sources.contains(&node) {
                "sour"
            } else if destination.is_some() && destination.unwrap() == node {
                "dest"
            } else if path.is_some() && traversing && path.unwrap().get(&node).is_some() {
                "visi"
            } else if path.is_some() && path.unwrap().get(&node).is_some() {
                "path"
            } else if is_traversed {
                "trav"
            } else if maze[node] > 0 {
                "open"
            } else {
                "bloc"
            };

            td_list.push(format!("<td class=\"{}\"></td>", class))
        }
        tr_list.push(format!("        <tr>{}</tr>", td_list.join("")));
    }
    format!("<table class=\"back\">\n    <tbody>{}\n    </tbody>\n</table>", tr_list.join("\n"))
}
