use std::collections::{HashMap, HashSet};

use crate::{graph::Graph, node::Node};

mod cid;
mod graph;
mod node;

fn main() {
    let mut graph = Graph::new();
    let mut map = HashMap::<u64, Node>::new();
    let mut g_set = HashSet::<u64>::new();

    println!("input operation(add, get_all)");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    if input == "add" {
        let mut payload = String::new();
        println!("input number");
        std::io::stdin().read_line(&mut payload).unwrap();
        let payload = payload.trim();
        let node = Node::new(payload.to_string());
        println!("node: {:?}", node);
        graph.add_node(node.cid.clone());
        map.insert(graph.get_nodes_len().try_into().unwrap(), node);
        println!("graph: {:?}", graph);
    }
}
