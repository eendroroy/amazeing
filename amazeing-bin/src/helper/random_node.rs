use amazeing::DNode;
use rand::random_range;

pub(crate) fn random_node((r, c): DNode) -> DNode {
    (random_range(0..r), random_range(0..c))
}
