use crate::Node;
use std::collections::BTreeMap;

pub(crate) fn reconstruct_path(destination: Node, parent: &BTreeMap<Node, Node>) -> Vec<Node> {
    let mut path = Vec::<Node>::new();
    let mut current_node = destination;
    while parent.contains_key(&current_node) {
        path.push(current_node);
        current_node = parent[&current_node];
    }
    path.push(current_node);
    path.reverse();
    path
}
