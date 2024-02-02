use std::collections::{HashMap, HashSet};

use crate::{cid::Cid, dag_syncer::DagSyncer, graph::Graph, node::Node};

#[derive(Debug, Clone)]
pub struct MerkleDag {
    pub graph: Graph,
}

impl MerkleDag {
    pub fn new() -> Self {
        Self {
            graph: Graph::new(),
        }
    }

    pub fn add_node(&mut self, payload: (String, i64), dag_pool: &mut DagSyncer) {
        let node = Node::new(payload, self.graph.get_nodes().clone());
        self.graph.delete_all_nodes();
        self.graph.add_node(node.cid.clone());
        dag_pool.map.insert(node.cid.clone(), node);
    }

    pub fn search(&self, dag_pool: &DagSyncer) -> HashSet<i64> {
        let mut used = HashSet::new();
        let mut result = HashSet::new();
        self.dfs(self.graph.get_nodes(), &mut used, &mut result, dag_pool);
        result
    }

    pub fn merge<'a>(&'a mut self, merkle_dag: &'a MerkleDag, dag_pool: &DagSyncer) {
        // rootが同じかの確認
        if self.graph.get_nodes() == merkle_dag.graph.get_nodes() {
            // 同じならマージしない
            return;
        }

        // rootが異なる場合

        // selfのrootがmerkle_dagに完全に含まれている場合
        // selfはmerkle_dagに完全に含まれている(部分木になっている)ためmerkle_dagを返す
        let mut used = HashSet::new();
        let mut merkle_dag_cid_set = HashSet::new();
        merkle_dag.dfs_cid(
            merkle_dag.graph.get_nodes(),
            &mut used,
            &mut merkle_dag_cid_set,
            &dag_pool,
        );
        let merkle_dag_cid_set_len = merkle_dag_cid_set.len();
        let mut cnt = 0;
        for merkle_dag_node_cid in merkle_dag_cid_set {
            for self_node_cid in self.graph.get_nodes() {
                if merkle_dag_node_cid == self_node_cid {
                    cnt += 1;
                }
            }
        }
        if cnt == merkle_dag_cid_set_len {
            *self = merkle_dag.clone();
            return;
        }

        // merkle_dagのrootがselfに完全に含まれている場合
        // merkle_dagはselfに完全に含まれている(部分木になっている)ためmerkle_dagを返す
        used = HashSet::new();
        let mut self_cid_set = HashSet::new();
        self.dfs_cid(
            self.graph.get_nodes(),
            &mut used,
            &mut self_cid_set,
            &dag_pool,
        );
        let self_cid_set_len = self_cid_set.len();
        cnt = 0;
        for self_node_cid in self_cid_set {
            for merkle_dag_node_cid in merkle_dag.graph.get_nodes() {
                if self_node_cid == merkle_dag_node_cid {
                    cnt += 1;
                }
            }
        }
        if cnt == self_cid_set_len {
            return;
        }

        // selfとmerkle_dagのノードの重複を探してマージする

        // merkle_dagのグラフをselfにコピー
        /* for merkle_dag_node in &merkle_dag.map {
            let parent_cid = merkle_dag_node.0;
            let parent_node = merkle_dag_node.1;
            // merged_dag.graph.add_node(parent_cid.clone());
            self.map.insert(parent_cid.clone(), parent_node.clone());
        } */
        for merkle_dag_node_cid in merkle_dag.graph.get_nodes() {
            self.graph.add_node(merkle_dag_node_cid.clone());
        }
    }

    fn dfs(
        &self,
        cids: &Vec<Cid>,
        used: &mut HashSet<Cid>,
        set: &mut HashSet<i64>,
        dag_pool: &DagSyncer,
    ) {
        for cid in cids {
            if used.contains(cid) {
                continue;
            }
            used.insert(cid.clone());
            let node = dag_pool.map.get(cid).unwrap();
            self.dfs(&node.child_cids, used, set, dag_pool);
            set.insert(node.payload.1);
        }
    }

    fn dfs_cid<'a>(
        &'a self,
        cids: &Vec<Cid>,
        used: &mut HashSet<Cid>,
        set: &mut HashSet<&'a Cid>,
        dag_pool: &'a DagSyncer,
    ) {
        for cid in cids {
            if used.contains(cid) {
                continue;
            }
            used.insert(cid.clone());
            let node = dag_pool.map.get(cid).unwrap();
            self.dfs_cid(&node.child_cids, used, set, dag_pool);
            set.insert(&node.cid);
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::{HashMap, HashSet};

    use crate::{dag_syncer::DagSyncer, node::Node};

    use super::MerkleDag;

    #[test]
    fn test_merkle_dag() {
        use crate::merkle_dag::MerkleDag;
        use crate::node::Node;

        let mut merkle_dag = MerkleDag::new();
        let mut dag_pool = DagSyncer::new();

        // Nodeを作成してDAGに挿入
        let node1 = Node::new(("add".to_string(), 1), Vec::new());
        merkle_dag.graph.add_node(node1.cid.clone());
        dag_pool.map.insert(node1.cid.clone(), node1.clone());
        let node2 = Node::new(("add".to_string(), 2), vec![node1.cid.clone()]);
        merkle_dag.graph.add_node(node2.cid.clone());
        dag_pool.map.insert(node2.cid.clone(), node2.clone());
        let node3 = Node::new(("add".to_string(), 3), Vec::new());
        merkle_dag.graph.add_node(node3.cid.clone());
        dag_pool.map.insert(node3.cid.clone(), node3.clone());
        let node4 = Node::new(
            ("add".to_string(), 4),
            vec![node2.cid.clone(), node3.cid.clone()],
        );
        merkle_dag.graph.add_node(node4.cid.clone());
        dag_pool.map.insert(node4.cid.clone(), node4.clone());
        let node5 = Node::new(("add".to_string(), 5), Vec::new());
        merkle_dag.graph.add_node(node5.cid.clone());
        dag_pool.map.insert(node5.cid.clone(), node5.clone());
        let node6 = Node::new(("add".to_string(), 6), Vec::new());
        merkle_dag.graph.add_node(node6.cid.clone());
        dag_pool.map.insert(node6.cid.clone(), node6.clone());
        let node7 = Node::new(
            ("add".to_string(), 7),
            vec![node5.cid.clone(), node4.cid.clone()],
        );
        merkle_dag.graph.add_node(node7.cid.clone());
        dag_pool.map.insert(node7.cid.clone(), node7);
        let node8 = Node::new(
            ("add".to_string(), 8),
            vec![node4.cid.clone(), node6.cid.clone()],
        );
        merkle_dag.graph.add_node(node8.cid.clone());
        dag_pool.map.insert(node8.cid.clone(), node8);

        // DAGを辿ってsetを作成
        let set = merkle_dag.search(&dag_pool);
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

        let mut dag_pool = DagSyncer::new();

        // Nodeを作成してDAGに挿入

        // DAG AがDAG Bの部分木である場合のテスト
        // DAG A
        let dag_a_node1 = Node::new(("add".to_string(), 2), Vec::new());
        let dag_a_node2 = Node::new(("add".to_string(), 3), Vec::new());
        let dag_a_node3 = Node::new(
            ("add".to_string(), 4),
            vec![dag_a_node1.cid.clone(), dag_a_node2.cid.clone()],
        );
        dag_pool
            .map
            .insert(dag_a_node1.cid.clone(), dag_a_node1.clone());
        dag_pool
            .map
            .insert(dag_a_node2.cid.clone(), dag_a_node2.clone());
        dag_pool
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
        dag_pool
            .map
            .insert(dag_b_node1.cid.clone(), dag_b_node1.clone());
        dag_pool
            .map
            .insert(dag_b_node2.cid.clone(), dag_b_node2.clone());
        dag_pool
            .map
            .insert(dag_b_node3.cid.clone(), dag_b_node3.clone());
        dag_pool
            .map
            .insert(dag_b_node4.cid.clone(), dag_b_node4.clone());
        dag_pool
            .map
            .insert(dag_b_node5.cid.clone(), dag_b_node5.clone());
        dag_b.graph.add_node(dag_b_node5.cid.clone());

        // println!("dag_a: {:?}", dag_a.search());
        dag_a.merge(&dag_b, &dag_pool);
        // println!("dag_b: {:?}", dag_b.search());
        // println!("dag_a(merged): {:?}", dag_a.search());

        let result1: HashSet<i64> = [1, 2, 3, 4, 5].iter().cloned().collect();
        assert_eq!(dag_a.search(&dag_pool), result1);

        // 同じDAGをマージした場合のテスト
        let dag_a_a = dag_a.clone();
        // println!("dag_a: {:?}", dag_a.search());
        dag_a.merge(&dag_a_a, &dag_pool);
        // println!("dag_a(merged): {:?}", dag_a.search());
        let result2: HashSet<i64> = [1, 2, 3, 4, 5].iter().cloned().collect();
        assert_eq!(dag_a.search(&dag_pool), result2);

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
        dag_pool
            .map
            .insert(dag_c_node1.cid.clone(), dag_c_node1.clone());
        dag_pool
            .map
            .insert(dag_c_node2.cid.clone(), dag_c_node2.clone());
        dag_pool
            .map
            .insert(dag_c_node3.cid.clone(), dag_c_node3.clone());
        dag_pool
            .map
            .insert(dag_c_node4.cid.clone(), dag_c_node4.clone());
        dag_pool
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
        dag_pool
            .map
            .insert(dag_d_node1.cid.clone(), dag_d_node1.clone());
        dag_pool
            .map
            .insert(dag_d_node2.cid.clone(), dag_d_node2.clone());
        dag_pool
            .map
            .insert(dag_d_node3.cid.clone(), dag_d_node3.clone());
        dag_pool
            .map
            .insert(dag_d_node4.cid.clone(), dag_d_node4.clone());
        dag_pool
            .map
            .insert(dag_d_node5.cid.clone(), dag_d_node5.clone());
        dag_d.graph.add_node(dag_d_node5.cid.clone());

        // println!("dag_c: {:?}", dag_c.search());

        dag_c.merge(&dag_d, &dag_pool);

        // println!("dag_d: {:?}", dag_d.search());
        // println!("dag_c(merged): {:?}", dag_c.search());

        let result3: HashSet<i64> = [1, 2, 3, 4, 5, 6, 7, 8].iter().cloned().collect();
        assert_eq!(dag_c.search(&dag_pool), result3);

        // DAG EとDAG Fに共通するノードがない場合のテスト
        // DAG E
        let dag_e_node1 = Node::new(("add".to_string(), 1), Vec::new());
        let dag_e_node2 = Node::new(("add".to_string(), 2), Vec::new());
        let dag_e_node3 = Node::new(
            ("add".to_string(), 3),
            vec![dag_e_node1.cid.clone(), dag_e_node2.cid.clone()],
        );
        dag_pool
            .map
            .insert(dag_e_node1.cid.clone(), dag_e_node1.clone());
        dag_pool
            .map
            .insert(dag_e_node2.cid.clone(), dag_e_node2.clone());
        dag_pool
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
        dag_pool
            .map
            .insert(dag_f_node1.cid.clone(), dag_f_node1.clone());
        dag_pool
            .map
            .insert(dag_f_node2.cid.clone(), dag_f_node2.clone());
        dag_pool
            .map
            .insert(dag_f_node3.cid.clone(), dag_f_node3.clone());
        dag_f.graph.add_node(dag_f_node3.cid.clone());

        // println!("dag_e: {:?}", dag_e.search());

        dag_e.merge(&dag_f, &dag_pool);

        // println!("dag_f: {:?}", dag_f.search());
        // println!("dag_e(merged): {:?}", dag_e.search());

        let result4: HashSet<i64> = [1, 2, 3, 4, 5, 6].iter().cloned().collect();
        assert_eq!(dag_e.search(&dag_pool), result4);

        // 複数のルートを持つ場合のテスト
        // DAG G
        let dag_g_node1 = Node::new(("add".to_string(), 1), Vec::new());
        let dag_g_node2 = Node::new(("add".to_string(), 2), vec![dag_g_node1.cid.clone()]);
        let dag_g_node3 = Node::new(("add".to_string(), 3), vec![dag_g_node1.cid.clone()]);
        dag_pool
            .map
            .insert(dag_g_node1.cid.clone(), dag_g_node1.clone());
        dag_pool
            .map
            .insert(dag_g_node2.cid.clone(), dag_g_node2.clone());
        dag_pool
            .map
            .insert(dag_g_node3.cid.clone(), dag_g_node3.clone());
        dag_g.graph.add_node(dag_g_node2.cid.clone());

        // DAG H
        let dag_h_node1 = Node::new(("add".to_string(), 4), Vec::new());
        let dag_h_node2 = Node::new(("add".to_string(), 5), vec![dag_h_node1.cid.clone()]);
        let dag_h_node3 = Node::new(("add".to_string(), 6), vec![dag_h_node1.cid.clone()]);
        dag_pool
            .map
            .insert(dag_h_node1.cid.clone(), dag_h_node1.clone());
        dag_pool
            .map
            .insert(dag_h_node2.cid.clone(), dag_h_node2.clone());
        dag_pool
            .map
            .insert(dag_h_node3.cid.clone(), dag_h_node3.clone());
        dag_h.graph.add_node(dag_h_node2.cid.clone());

        // println!("dag_g: {:?}", dag_g.search());

        dag_g.merge(&dag_h, &dag_pool);
        // println!("dag_h: {:?}", dag_h.search());
        // println!("dag_g(merged): {:?}", dag_g.search());

        let result5: HashSet<i64> = [1, 2, 4, 5].iter().cloned().collect();
        assert_eq!(dag_g.search(&dag_pool), result5);
    }
}
