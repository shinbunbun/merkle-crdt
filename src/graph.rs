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
    pub fn pop_node(&mut self) -> Option<Cid> {
        self.nodes.pop()
    }
    pub fn get_nodes(&self) -> &Vec<Cid> {
        &self.nodes
    }
}
