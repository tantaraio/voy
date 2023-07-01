mod engine;

#[cfg(test)]
mod tests;

pub use engine::{add, clear, index, remove, search, Index, Query};
