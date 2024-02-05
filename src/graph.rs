use std::collections::BTreeSet;

use crate::cid::Cid;

#[derive(Debug, Clone)]
pub struct Graph {
    nodes: BTreeSet<Cid>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: BTreeSet::new(),
        }
    }
    pub fn add_node(&mut self, cid: Cid) -> bool {
        self.nodes.insert(cid)
    }
    pub fn get_nodes(&self) -> &BTreeSet<Cid> {
        &self.nodes
    }
    pub fn delete_all_nodes(&mut self) {
        self.nodes = BTreeSet::new();
    }
    pub fn search(&self, cid: Cid) -> bool {
        self.nodes.contains(&cid)
    }
}
