use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn hash<T: Hash>(target: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    target.hash(&mut hasher);

    hasher.finish()
}
