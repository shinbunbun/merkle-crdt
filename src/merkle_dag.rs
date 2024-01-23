use std::collections::{HashMap, HashSet};

use crate::{cid::Cid, graph::Graph, node::Node};

#[derive(Debug, Clone)]
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

        // rootが異なる場合

        // selfのNodeとmerkle_dagのNodeのidと値を全てとってくる
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

        // selfのrootがmerkle_dagに完全に含まれている場合
        // selfはmerkle_dagに完全に含まれている(部分木になっている)ためmerkle_dagを返す
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

        // merkle_dagのrootがselfに完全に含まれている場合
        // merkle_dagはselfに完全に含まれている(部分木になっている)ためmerkle_dagを返す
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
        }

        // selfとmerkle_dagのノードの重複を探してマージする

        let mut merged_dag = MerkleDag::new();

        // selfのグラフをmerged_dagにコピー
        for self_node in &self.map {
            let parent_cid = self_node.0;
            let parent_node = self_node.1;
            // merged_dag.graph.add_node(parent_cid.clone());
            merged_dag
                .map
                .insert(parent_cid.clone(), parent_node.clone());
        }
        for self_node_cid in self.graph.get_nodes() {
            merged_dag.graph.add_node(self_node_cid.clone());
        }

        // merkle_dagのグラフをmerged_dagにコピー
        for merkle_dag_node in &merkle_dag.map {
            let parent_cid = merkle_dag_node.0;
            let parent_node = merkle_dag_node.1;
            // merged_dag.graph.add_node(parent_cid.clone());
            merged_dag
                .map
                .insert(parent_cid.clone(), parent_node.clone());
        }
        for merkle_dag_node_cid in merkle_dag.graph.get_nodes() {
            merged_dag.graph.add_node(merkle_dag_node_cid.clone());
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
    use crate::node::Node;

    use super::MerkleDag;

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

    #[test]
    fn test_merge() {
        let mut dag_a = MerkleDag::new();
        let mut dag_b = MerkleDag::new();
        let mut dag_c = MerkleDag::new();
        let mut dag_d = MerkleDag::new();
        let mut dag_e = MerkleDag::new();
        let mut dag_f = MerkleDag::new();
        let mut dag_g = MerkleDag::new();
        let mut dag_h = MerkleDag::new();

        // Nodeを作成してDAGに挿入

        // DAG AがDAG Bの部分木である場合のテスト
        // DAG A
        let dag_a_node1 = Node::new(("add".to_string(), 2), Vec::new());
        let dag_a_node2 = Node::new(("add".to_string(), 3), Vec::new());
        let dag_a_node3 = Node::new(
            ("add".to_string(), 4),
            vec![dag_a_node1.cid.clone(), dag_a_node2.cid.clone()],
        );
        dag_a
            .map
            .insert(dag_a_node1.cid.clone(), dag_a_node1.clone());
        dag_a
            .map
            .insert(dag_a_node2.cid.clone(), dag_a_node2.clone());
        dag_a
            .map
            .insert(dag_a_node3.cid.clone(), dag_a_node3.clone());
        dag_a.graph.add_node(dag_a_node3.cid.clone());

        // DAG B
        let dag_b_node1 = Node::new(("add".to_string(), 2), Vec::new());
        let dag_b_node2 = Node::new(("add".to_string(), 3), Vec::new());
        let dag_b_node3 = Node::new(
            ("add".to_string(), 1),
            vec![dag_b_node1.cid.clone(), dag_b_node2.cid.clone()],
        );
        let dag_b_node4 = Node::new(("add".to_string(), 5), Vec::new());
        let dag_b_node5 = Node::new(
            ("add".to_string(), 4),
            vec![dag_b_node3.cid.clone(), dag_b_node4.cid.clone()],
        );
        dag_b
            .map
            .insert(dag_b_node1.cid.clone(), dag_b_node1.clone());
        dag_b
            .map
            .insert(dag_b_node2.cid.clone(), dag_b_node2.clone());
        dag_b
            .map
            .insert(dag_b_node3.cid.clone(), dag_b_node3.clone());
        dag_b
            .map
            .insert(dag_b_node4.cid.clone(), dag_b_node4.clone());
        dag_b
            .map
            .insert(dag_b_node5.cid.clone(), dag_b_node5.clone());
        dag_b.graph.add_node(dag_b_node5.cid.clone());

        let dag_a_b = dag_a.merge(&dag_b);

        println!("dag_a: {:?}", dag_a.search());
        println!("dag_b: {:?}", dag_b.search());
        println!("dag_a_b: {:?}", dag_a_b.search());

        // 同じDAGをマージした場合のテスト
        let dag_a_a = dag_a.merge(&dag_a);

        println!("dag_a: {:?}", dag_a.search());
        println!("dag_a_a: {:?}", dag_a_a.search());

        // DAG CとDAG Dに共通するノードがある場合のテスト
        // DAG C
        let dag_c_node1 = Node::new(("add".to_string(), 4), Vec::new());
        let dag_c_node2 = Node::new(("add".to_string(), 5), Vec::new());
        let dag_c_node3 = Node::new(
            ("add".to_string(), 2),
            vec![dag_c_node1.cid.clone(), dag_c_node2.cid.clone()],
        );
        let dag_c_node4 = Node::new(("add".to_string(), 3), Vec::new());
        let dag_c_node5 = Node::new(
            ("add".to_string(), 1),
            vec![dag_c_node3.cid.clone(), dag_c_node4.cid.clone()],
        );
        dag_c
            .map
            .insert(dag_c_node1.cid.clone(), dag_c_node1.clone());
        dag_c
            .map
            .insert(dag_c_node2.cid.clone(), dag_c_node2.clone());
        dag_c
            .map
            .insert(dag_c_node3.cid.clone(), dag_c_node3.clone());
        dag_c
            .map
            .insert(dag_c_node4.cid.clone(), dag_c_node4.clone());
        dag_c
            .map
            .insert(dag_c_node5.cid.clone(), dag_c_node5.clone());
        dag_c.graph.add_node(dag_c_node5.cid.clone());

        // DAG D
        let dag_d_node1 = Node::new(("add".to_string(), 8), Vec::new());
        let dag_d_node2 = Node::new(("add".to_string(), 5), Vec::new());
        let dag_d_node3 = Node::new(
            ("add".to_string(), 7),
            vec![dag_d_node1.cid.clone(), dag_d_node2.cid.clone()],
        );
        let dag_d_node4 = Node::new(("add".to_string(), 4), Vec::new());
        let dag_d_node5 = Node::new(
            ("add".to_string(), 6),
            vec![dag_d_node3.cid.clone(), dag_d_node4.cid.clone()],
        );
        dag_d
            .map
            .insert(dag_d_node1.cid.clone(), dag_d_node1.clone());
        dag_d
            .map
            .insert(dag_d_node2.cid.clone(), dag_d_node2.clone());
        dag_d
            .map
            .insert(dag_d_node3.cid.clone(), dag_d_node3.clone());
        dag_d
            .map
            .insert(dag_d_node4.cid.clone(), dag_d_node4.clone());
        dag_d
            .map
            .insert(dag_d_node5.cid.clone(), dag_d_node5.clone());
        dag_d.graph.add_node(dag_d_node5.cid.clone());

        let dag_c_d = dag_c.merge(&dag_d);

        println!("dag_c: {:?}", dag_c.search());
        println!("dag_d: {:?}", dag_d.search());
        println!("dag_c_d: {:?}", dag_c_d.search());

        // DAG EとDAG Fに共通するノードがない場合のテスト
        // DAG E
        let dag_e_node1 = Node::new(("add".to_string(), 1), Vec::new());
        let dag_e_node2 = Node::new(("add".to_string(), 2), Vec::new());
        let dag_e_node3 = Node::new(
            ("add".to_string(), 3),
            vec![dag_e_node1.cid.clone(), dag_e_node2.cid.clone()],
        );
        dag_e
            .map
            .insert(dag_e_node1.cid.clone(), dag_e_node1.clone());
        dag_e
            .map
            .insert(dag_e_node2.cid.clone(), dag_e_node2.clone());
        dag_e
            .map
            .insert(dag_e_node3.cid.clone(), dag_e_node3.clone());
        dag_e.graph.add_node(dag_e_node3.cid.clone());

        // DAG F
        let dag_f_node1 = Node::new(("add".to_string(), 4), Vec::new());
        let dag_f_node2 = Node::new(("add".to_string(), 5), Vec::new());
        let dag_f_node3 = Node::new(
            ("add".to_string(), 6),
            vec![dag_f_node1.cid.clone(), dag_f_node2.cid.clone()],
        );
        dag_f
            .map
            .insert(dag_f_node1.cid.clone(), dag_f_node1.clone());
        dag_f
            .map
            .insert(dag_f_node2.cid.clone(), dag_f_node2.clone());
        dag_f
            .map
            .insert(dag_f_node3.cid.clone(), dag_f_node3.clone());
        dag_f.graph.add_node(dag_f_node3.cid.clone());

        let dag_e_f = dag_e.merge(&dag_f);

        println!("dag_e: {:?}", dag_e.search());
        println!("dag_f: {:?}", dag_f.search());
        println!("dag_e_f: {:?}", dag_e_f.search());

        // 複数のルートを持つ場合のテスト
        // DAG G
        let dag_g_node1 = Node::new(("add".to_string(), 1), Vec::new());
        let dag_g_node2 = Node::new(("add".to_string(), 2), vec![dag_g_node1.cid.clone()]);
        let dag_g_node3 = Node::new(("add".to_string(), 3), vec![dag_g_node1.cid.clone()]);
        dag_g
            .map
            .insert(dag_g_node1.cid.clone(), dag_g_node1.clone());
        dag_g
            .map
            .insert(dag_g_node2.cid.clone(), dag_g_node2.clone());
        dag_g
            .map
            .insert(dag_g_node3.cid.clone(), dag_g_node3.clone());
        dag_g.graph.add_node(dag_g_node2.cid.clone());

        // DAG H
        let dag_h_node1 = Node::new(("add".to_string(), 4), Vec::new());
        let dag_h_node2 = Node::new(("add".to_string(), 5), vec![dag_h_node1.cid.clone()]);
        let dag_h_node3 = Node::new(("add".to_string(), 6), vec![dag_h_node1.cid.clone()]);
        dag_h
            .map
            .insert(dag_h_node1.cid.clone(), dag_h_node1.clone());
        dag_h
            .map
            .insert(dag_h_node2.cid.clone(), dag_h_node2.clone());
        dag_h
            .map
            .insert(dag_h_node3.cid.clone(), dag_h_node3.clone());
        dag_h.graph.add_node(dag_h_node2.cid.clone());

        let dag_g_h = dag_g.merge(&dag_h);

        println!("dag_g: {:?}", dag_g.search());
        println!("dag_h: {:?}", dag_h.search());
        println!("dag_g_h: {:?}", dag_g_h.search());
    }
}
