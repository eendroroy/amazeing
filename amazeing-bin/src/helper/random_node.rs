use amazeing::Node;
use rand::random_range;

pub(crate) fn random_node((r, c): Node) -> Node {
    (random_range(0..r), random_range(0..c))
}
