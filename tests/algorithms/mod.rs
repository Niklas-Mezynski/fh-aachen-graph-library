use graph_library::graph::{WeightedEdge, WithID};

pub mod count_connected_subgraphs;
pub mod mst;
pub mod tsp;

/// Vertex representation for testing, implements the required traits
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestVertex(pub usize);

impl WithID for TestVertex {
    type IDType = usize;

    fn get_id(&self) -> Self::IDType {
        self.0
    }
}

/// Edge representation for testing, implements weighted edge trait
#[derive(Debug, Clone, PartialEq)]
pub struct TestEdge(pub f64);

impl WeightedEdge for TestEdge {
    type WeightType = f64;

    fn get_weight(&self) -> Self::WeightType {
        self.0
    }
}
