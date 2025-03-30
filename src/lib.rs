pub mod algorithms;
pub mod graph;

// Re-export main types and traits
pub use crate::graph::traits::{GraphInterface, WithID};
pub use crate::graph::Graph;
