use crate::cid::CID;
use serde::Serialize;

#[derive(Debug)]
pub struct Node {
    pub cid: String,
    pub payload: String,
    pub child_cids: Vec<CID>,
}

#[derive(Serialize)]
struct ForHashNode {
    pub payload: String,
    pub child_cids: Vec<CID>,
}

impl Node {
    pub fn new(payload: String) -> Node {
        let for_hash_node = ForHashNode {
            payload: payload.clone(),
            child_cids: Vec::new(),
        };
        let json = serde_json::to_string(&for_hash_node).unwrap();
        let cid = sha256::digest(json.as_bytes());
        Node {
            cid,
            payload,
            child_cids: Vec::new(),
        }
    }
}
