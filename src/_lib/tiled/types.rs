use crate::_lib::tiled::node::Node;
use std::collections::{HashMap, VecDeque};

pub type MazeData = Vec<Vec<i8>>;

pub type NodeHeuFn = fn(Node, Node) -> u32;

pub type Rank = i32;
pub type Trace = HashMap<Node, Rank>;
pub type Tracer = Vec<Trace>;

pub(crate) type Push = fn(&mut VecDeque<Node>, Node);
pub(crate) type Pop = fn(&mut VecDeque<Node>) -> Option<Node>;
