use std::collections::{HashMap, HashSet};

use crate::{graph::Graph, node::Node};

mod cid;
mod graph;
mod node;

fn main() {
    // 必要なデータ構造の定義
    let mut graph = Graph::new();
    let mut map = HashMap::<u64, Node>::new();
    let mut g_set = HashSet::<i64>::new();

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

            // Nodeを作成
            let node = Node::new(("add".to_string(), value));
            println!("node: {:?}", node);

            // GraphにNodeのCIDを追加
            graph.add_node(node.cid.clone());

            // HashMapにNodeを追加
            // これをすることによりCIDからNodeを引くことができるようになる
            map.insert(graph.get_nodes_len().try_into().unwrap(), node);
            println!("graph: {:?}", graph);

            // g-setにValueを追加
            g_set.insert(value);
            println!("g_set: {:?}", g_set);
        }
    }
}
