use crate::DNode;
use std::collections::BTreeMap;

pub(crate) fn reconstruct_path(destination: DNode, parent: &BTreeMap<DNode, DNode>) -> Vec<DNode> {
    let mut path = Vec::<DNode>::new();
    let mut current_node = destination;
    while parent.contains_key(&current_node) {
        path.push(current_node);
        current_node = parent[&current_node];
    }
    path.push(current_node);
    path.reverse();
    path
}
