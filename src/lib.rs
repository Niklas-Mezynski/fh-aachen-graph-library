//! # Examples
//!
//! ```rust
//! use graph_library::{
//!     graph::{GraphBase, ListGraphBackend, WeightedEdge, WithID},
//!     ListGraph, Undirected,
//! };
//!
//! // Vertices have to be identifiable through some hashable ID
//! // (or even indexable when using the Matrix backend)
//! #[derive(Clone)]
//! struct Vertex(u32);
//! impl WithID for Vertex {
//!     type IDType = u32;
//!
//!     fn get_id(&self) -> Self::IDType {
//!         self.0
//!     }
//! }
//!
//! // Edge data can be anything
//! // When using algorithms that require weights, it has to implement the `WeightedEdge` trait
//! #[derive(Clone)]
//! struct Edge(f32);
//! impl WeightedEdge for Edge {
//!     type WeightType = f32;
//!
//!     fn get_weight(&self) -> Self::WeightType {
//!         self.0
//!     }
//! }
//!
//! let mut graph = ListGraph::<Vertex, Edge, Undirected>::new();
//!
//! graph.push_vertex(Vertex(1)).unwrap();
//! graph.push_vertex(Vertex(2)).unwrap();
//! graph.push_vertex(Vertex(3)).unwrap();
//! graph.push_vertex(Vertex(4)).unwrap();
//!
//! graph.push_edge(1, 2, Edge(1.0)).unwrap();
//! graph.push_edge(1, 3, Edge(4.0)).unwrap();
//! graph.push_edge(2, 3, Edge(2.0)).unwrap();
//! graph.push_edge(2, 4, Edge(3.0)).unwrap();
//! graph.push_edge(3, 4, Edge(1.0)).unwrap();
//! // graph:
//! //
//! //     1
//! //    / \
//! // 1.0   4.0
//! //  /     \
//! // 2--2.0--3
//! //  \     /
//! // 3.0  1.0
//! //    \ /
//! //     4
//! //
//!
//! // Construct an mst starting at vertex `1`.
//! // The resulting graph should also be a ListGraph
//! let mst = graph
//!     .mst_prim::<ListGraphBackend<_, _, Undirected>>(Some(1))
//!     .unwrap();
//!
//! let dfs_vec = mst
//!     .dfs_iter(1)
//!     .unwrap()
//!     .map(|v| v.get_id())
//!     .collect::<Vec<_>>();
//!
//! assert_eq!(dfs_vec, vec![1, 2, 3, 4]);
//! ```

pub mod algorithms;
pub mod graph;

// Re-export main types and traits
pub use crate::graph::error::GraphError;
pub use crate::graph::{Directed, Direction, Graph, ListGraph, Undirected};
