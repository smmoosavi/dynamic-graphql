use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn get_interface_hash(name: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    "interface:".hash(&mut hasher);
    name.hash(&mut hasher);
    hasher.finish()
}
