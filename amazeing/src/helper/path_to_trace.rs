use amazeing::matrix::Node;
use std::collections::HashMap;

pub(crate) fn path_to_trace(path: Vec<Node>) -> HashMap<Node, bool> {
    path.iter().map(|node| (*node, true)).collect()
}
