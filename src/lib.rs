pub mod algorithms;
pub mod graph;

// Re-export main types and traits
pub use crate::graph::error::GraphError;
pub use crate::graph::{Directed, Direction, Graph, ListGraph, Undirected};
