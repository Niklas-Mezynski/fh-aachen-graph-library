pub mod algorithms;
pub mod graph;

// Re-export main types and traits
pub use crate::graph::error::GraphError;
pub use crate::graph::traits::GraphInterface;
pub use crate::graph::Graph;
