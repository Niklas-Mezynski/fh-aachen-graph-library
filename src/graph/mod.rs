#[allow(clippy::module_inception)]
mod graph;

mod adjacency_list;
pub mod error;
pub mod traits;

pub use graph::{Graph, Vertex, VertexIDType};
pub use traits::{GraphInterface, WithID};
