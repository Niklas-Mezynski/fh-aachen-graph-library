#[allow(clippy::module_inception)]
mod graph;

mod adjacency_list;
mod adjacency_matrix;
pub mod error;
mod graph_structs;
mod traits;

pub use graph::Graph;
pub use graph_structs::{EdgeWeight, EdgeWithWeight, Vertex, VertexIDType};
pub use traits::*;
