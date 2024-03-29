use dag_syncer::DagSyncer;
use merkle_dag::MerkleDag;

mod cid;
mod dag_syncer;
mod graph;
mod merkle_dag;
mod node;

fn main() {
    // 必要なデータ構造の定義
    let mut merkle_dag = MerkleDag::new();
    let mut dag_pool = DagSyncer::new();

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

            merkle_dag.add_node(("add".to_string(), value), &mut dag_pool);
            println!("graph: {:?}", merkle_dag.graph);
        } else if input == "lookup" {
            // グラフを辿ってsetを作成
            let set = merkle_dag.search(&dag_pool);
            println!("set: {:?}", set);
        }
    }
}
