use std::collections::{HashMap, HashSet};

use cid::Cid;
use merkle_dag::MerkleDag;

use crate::node::Node;

mod cid;
mod graph;
mod merkle_dag;
mod node;

fn main() {
    // 必要なデータ構造の定義
    let mut merkle_dag = MerkleDag::new();

    loop {
        println!("input operation(add, lookup)");

        // 標準入力からの操作を受け付ける
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        // 操作が"add"だった場合
        if input == "add" {
            // 標準入力からの値を受け付ける
            let mut value = String::new();
            println!("input number");
            std::io::stdin().read_line(&mut value).unwrap();
            let value = value.trim().parse::<i64>().unwrap();

            merkle_dag.add_node(("add".to_string(), value));
            println!("graph: {:?}", merkle_dag.graph);
        } else if input == "lookup" {
            // グラフを辿ってsetを作成
            let set = merkle_dag.search();
            println!("set: {:?}", set);
        }
    }
}

// 子ノードを辿ってsetを作成する関数
fn search_child(child_cid: &Cid, map: &HashMap<Cid, Node>, set: &mut HashSet<i64>) {
    let child_node = map.get(child_cid).unwrap();
    let value = child_node.payload.1;
    set.insert(value);
    if let Some(cid) = child_node.child_cids.last() {
        search_child(cid, map, set)
    }
}
