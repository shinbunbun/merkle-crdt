use crate::cid::Cid;

#[derive(Debug, Clone)]
pub struct Graph {
    //TODO: Setにする？
    nodes: Vec<Cid>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph { nodes: Vec::new() }
    }
    pub fn add_node(&mut self, cid: Cid) {
        // 重複ノードの場合は追加しない
        if self.search_node(&cid).is_some() {
            return;
        }
        self.nodes.push(cid);
    }
    pub fn delete_all_nodes(&mut self) {
        self.nodes = Vec::new();
    }
    pub fn get_nodes(&self) -> &Vec<Cid> {
        &self.nodes
    }
    fn search_node(&self, cid: &Cid) -> Option<usize> {
        self.nodes.iter().position(|x| x == cid)
    }
}
