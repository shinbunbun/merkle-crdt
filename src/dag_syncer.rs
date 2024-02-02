use std::collections::HashMap;

use crate::{cid::Cid, node::Node};

pub struct DagSyncer {
    pub map: HashMap<Cid, Node>,
}

impl DagSyncer {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}
