use std::collections::HashMap;
use amazeing::matrix::Node;

pub(crate) fn path_to_trace(path: Vec<Node>) -> HashMap<Node, bool> {
    path.iter().map(|node| (*node, true)).collect()
}