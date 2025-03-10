use amazeing::DNode;

pub(crate) fn parse_node(node: &str) -> DNode {
    let node_data = node.split(",").collect::<Vec<&str>>();
    (
        u32::from_str_radix(node_data.get(0).unwrap(), 10).unwrap() as usize,
        u32::from_str_radix(node_data.get(1).unwrap(), 10).unwrap() as usize,
    )
}
