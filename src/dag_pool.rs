use std::collections::HashMap;

use crate::{cid::Cid, node::Node};

pub struct DagPool {
    pub map: HashMap<Cid, Node>,
}

impl DagPool {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}
