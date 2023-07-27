mod engine;
mod hash;

#[cfg(test)]
mod tests;

pub use engine::{add, clear, index, remove, search, size, Index, Query};
pub use hash::hash;
