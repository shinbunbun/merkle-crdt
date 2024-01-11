use crate::cid::Cid;

#[derive(Debug)]
pub struct Graph {
    nodes: Vec<Cid>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph { nodes: Vec::new() }
    }
    pub fn add_node(&mut self, cid: Cid) {
        self.nodes.push(cid);
    }
    pub fn get_nodes_len(&self) -> usize {
        self.nodes.len()
    }
}
