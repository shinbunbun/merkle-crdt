use std::collections::HashMap;

use crate::{cid::Cid, node::Node};

pub struct DagSyncer {
    map: HashMap<Cid, Node>,
}

impl DagSyncer {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    pub fn get_node(&self, cid: &Cid) -> Option<&Node> {
        self.map.get(cid)
    }
    pub fn put_node(&mut self, cid: Cid, node: Node) {
        self.map.insert(cid, node);
    }
}
