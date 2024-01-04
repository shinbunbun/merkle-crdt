pub type CID = String;

pub struct Node {
    pub cid: String,
    pub payload: String,
    pub child_cids: Vec<CID>,
}

pub struct Graph {
    nodes: Vec<CID>,
}
