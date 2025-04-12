#[allow(clippy::module_inception)]
mod graph;

mod adjacency_list;
pub mod error;
mod graph_structs;
mod traits;

pub use graph::Graph;
pub use graph_structs::{EdgeWeight, Vertex, VertexIDType, WeightedEdge};
pub use traits::{GraphInterface, WithID};
