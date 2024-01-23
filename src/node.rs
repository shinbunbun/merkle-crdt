use crate::cid::Cid;
use serde::Serialize;

pub type Payload = (String, i64);

#[derive(Debug, Clone)]
pub struct Node {
    pub cid: Cid,
    pub payload: Payload,
    pub child_cids: Vec<Cid>,
}

#[derive(Serialize)]
struct ForHashNode {
    pub payload: Payload,
    pub child_cids: Vec<Cid>,
}

impl Node {
    pub fn new(payload: Payload, child_cids: Vec<Cid>) -> Node {
        let for_hash_node = ForHashNode {
            payload: payload.clone(),
            child_cids: child_cids.clone(),
        };
        let json = serde_json::to_string(&for_hash_node).unwrap();
        let cid = sha256::digest(json.as_bytes());
        Node {
            cid,
            payload,
            child_cids,
        }
    }
}
