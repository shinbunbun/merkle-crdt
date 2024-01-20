use std::{
    collections::{HashMap, HashSet},
    usize,
};

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

    pub fn search(&self) -> HashSet<i64> {
        let mut used = HashSet::new();
        let result = self.dfs(self.graph.get_nodes(), &mut used);
        result
    }
    // pub fn merge(&self, merkle_dag: &MerkleDag) -> Self {

    // }

    fn dfs(&self, cids: &Vec<Cid>, used: &mut HashSet<Cid>) -> HashSet<i64> {
        let mut set = HashSet::new();
        for cid in cids {
            if used.contains(cid) {
                continue;
            }
            used.insert(cid.clone());
            let node = self.map.get(cid).unwrap();
            let mut child_set = self.dfs(&node.child_cids, used);
            set.insert(node.payload.1);
            set.extend(child_set.drain());
        }
        set
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_merkle_dag() {
        use crate::merkle_dag::MerkleDag;
        use crate::node::Node;

        let mut merkle_dag = MerkleDag::new();

        // Nodeを作成してDAGに挿入
        let node1 = Node::new(("add".to_string(), 1), Vec::new());
        merkle_dag.graph.add_node(node1.cid.clone());
        merkle_dag.map.insert(node1.cid.clone(), node1.clone());
        let node2 = Node::new(("add".to_string(), 2), vec![node1.cid.clone()]);
        merkle_dag.graph.add_node(node2.cid.clone());
        merkle_dag.map.insert(node2.cid.clone(), node2.clone());
        let node3 = Node::new(("add".to_string(), 3), Vec::new());
        merkle_dag.graph.add_node(node3.cid.clone());
        merkle_dag.map.insert(node3.cid.clone(), node3.clone());
        let node4 = Node::new(
            ("add".to_string(), 4),
            vec![node2.cid.clone(), node3.cid.clone()],
        );
        merkle_dag.graph.add_node(node4.cid.clone());
        merkle_dag.map.insert(node4.cid.clone(), node4.clone());
        let node5 = Node::new(("add".to_string(), 5), Vec::new());
        merkle_dag.graph.add_node(node5.cid.clone());
        merkle_dag.map.insert(node5.cid.clone(), node5.clone());
        let node6 = Node::new(("add".to_string(), 6), Vec::new());
        merkle_dag.graph.add_node(node6.cid.clone());
        merkle_dag.map.insert(node6.cid.clone(), node6.clone());
        let node7 = Node::new(
            ("add".to_string(), 7),
            vec![node5.cid.clone(), node4.cid.clone()],
        );
        merkle_dag.graph.add_node(node7.cid.clone());
        merkle_dag.map.insert(node7.cid.clone(), node7);
        let node8 = Node::new(
            ("add".to_string(), 8),
            vec![node4.cid.clone(), node6.cid.clone()],
        );
        merkle_dag.graph.add_node(node8.cid.clone());
        merkle_dag.map.insert(node8.cid.clone(), node8);

        // DAGを辿ってsetを作成
        let set = merkle_dag.search();
        println!("set: {:?}", set);
    }
}
