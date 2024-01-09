use crate::cid::CID;

#[derive(Debug)]
pub struct Graph {
    nodes: Vec<CID>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph { nodes: Vec::new() }
    }
    pub fn add_node(&mut self, cid: CID) {
        self.nodes.push(cid);
    }
    pub fn get_nodes_len(&self) -> usize {
        self.nodes.len()
    }
}
