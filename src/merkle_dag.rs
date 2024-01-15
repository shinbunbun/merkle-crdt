use std::collections::HashMap;

use crate::{cid::Cid, graph::Graph, node::Node};

pub struct MerkleDag {
    pub graph: Graph,
    pub map: HashMap<Cid, Node>,
}

impl MerkleDag {
    pub fn new() -> Self {
        Self {
            graph: Graph::new(),
            map: HashMap::new(),
        }
    }
}
