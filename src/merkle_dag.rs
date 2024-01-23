use std::collections::{HashMap, HashSet};

use crate::{cid::Cid, graph::Graph, node::Node};

#[derive(Clone)]
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

    pub fn add_node(&mut self, node: Node) {
        self.graph.add_node(node.cid.clone());
        self.map.insert(node.cid.clone(), node);
    }

    pub fn search(&self) -> HashSet<i64> {
        let mut used = HashSet::new();
        let result = self.dfs(self.graph.get_nodes(), &mut used);
        result
    }

    pub fn merge<'a>(&'a self, merkle_dag: &'a MerkleDag) -> Self {
        // rootが同じかの確認
        if self.graph.get_nodes() == merkle_dag.graph.get_nodes() {
            // 同じならマージしない
            return self.clone();
        }
        /* // rootが異なる場合
        // merkle_dagのnodeにselfのrootが含まれる場合、merkle_dagを返す
        for root_cid in self.graph.get_nodes() {
            if merkle_dag.graph.get_nodes().contains(root_cid) {
                return merkle_dag;
            }
        }
        // selfのnodeにmerkle_dagのrootが含まれる場合、selfを返す
        for root_cid in merkle_dag.graph.get_nodes() {
            if self.graph.get_nodes().contains(root_cid) {
                return self;
            }
        } */

        // rootが異なる場合
        // selfとmerkle_dagのノードの重複を探してマージする

        /*  // selfのNodeとmerkle_dagのNodeのidと値を全てとってくる
        let self_cid_val_set = self
            .map
            .iter()
            .map(|(cid, node)| (cid, node.payload.1))
            .collect::<Vec<(&Cid, i64)>>();
        let merkle_dag_cid_val_set = merkle_dag
            .map
            .iter()
            .map(|(cid, node)| (cid, node.payload.1))
            .collect::<Vec<(&Cid, i64)>>();

        // selfがmerkle_dagの部分木になっている場合
        // selfはmerkle_dagに完全に含まれているためmerkle_dagを返す
        let merkle_dag_cid_val_set_len = merkle_dag_cid_val_set.len();
        let mut cnt = 0;
        for merkle_dag_node in merkle_dag_cid_val_set {
            for self_node in self.graph.get_nodes() {
                if merkle_dag_node.0 == self_node {
                    cnt += 1;
                }
            }
        }
        if cnt == merkle_dag_cid_val_set_len {
            return merkle_dag.clone();
        }

        // merkle_dagがselfの部分木になっている場合
        // merkle_dagはselfに完全に含まれているためmerkle_dagを返す
        let self_cid_val_set_len = self_cid_val_set.len();
        cnt = 0;
        for self_node in self_cid_val_set {
            for merkle_dag_node in merkle_dag.graph.get_nodes() {
                if self_node.0 == merkle_dag_node {
                    cnt += 1;
                }
            }
        }
        if cnt == self_cid_val_set_len {
            return self.clone();
        } */

        // selfとmerkle_dagのノードの重複を探してマージする

        let mut merged_dag = MerkleDag::new();

        // selfのグラフをmerged_dagにコピー
        for self_node in &self.map {
            let parent_cid = self_node.0;
            let parent_node = self_node.1;
            merged_dag.graph.add_node(parent_cid.clone());
            merged_dag
                .map
                .insert(parent_cid.clone(), parent_node.clone());
        }

        // merkle_dagのグラフをmerged_dagにコピー
        for merkle_dag_node in &merkle_dag.map {
            let parent_cid = merkle_dag_node.0;
            let parent_node = merkle_dag_node.1;
            merged_dag.graph.add_node(parent_cid.clone());
            merged_dag
                .map
                .insert(parent_cid.clone(), parent_node.clone());
        }

        merged_dag
    }

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
