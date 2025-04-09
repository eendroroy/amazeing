use amazeing::matrix::{Node, Trace};

pub(crate) fn path_to_trace(path: Vec<Node>) -> Trace {
    let mut rank = i32::MAX;
    path.into_iter()
        .map(|node| {
            rank -= 1;
            (node, rank + 1)
        })
        .collect()
}
