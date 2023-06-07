mod engine;

#[cfg(test)]
mod tests;

pub use engine::{add, index, remove, search, Index, Query};
