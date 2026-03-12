use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct ConsistentHasher {
    nodes: Vec<String>,
    #[allow(dead_code)]
    vnodes: usize,
}

impl ConsistentHasher {
    pub fn new(nodes: Vec<String>, vnodes: usize) -> Self {
        Self {
            nodes,
            vnodes,
        }
    }

    pub fn get_shard(&self, key: &str) -> String {
        if self.nodes.is_empty() {
            return "local".to_string();
        }

        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish();

        // Simple modulo for now (Rendezvous Hashing or Ring would be better for full prod)
        // But this demonstrates the routing logic.
        let idx = (hash as usize) % self.nodes.len();
        self.nodes[idx].clone()
    }
}
