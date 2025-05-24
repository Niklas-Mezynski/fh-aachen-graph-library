use algorithms::*;
use criterion::{criterion_group, criterion_main, Criterion};
use graph::*;
use graph_library::graph::{WeightedEdge, WithID};

mod algorithms;
mod graph;

/// Configure and run benchmarks
fn configure() -> Criterion {
    Criterion::default()
        .sample_size(10) // Reduced sample size for faster development iterations
        .measurement_time(std::time::Duration::from_secs(5)) // Reasonable measurement time
        .warm_up_time(std::time::Duration::from_secs(1)) // Warm up before actual measurements
}

criterion_main!(benches);
criterion_group!(
    name = benches;
    config = configure();
    targets =
        creation::graph_creation,
        count_connected_subgraphs::count_connected_subgraphs,
        mst::mst,
        tsp::tsp,
        shortest_path::shortest_path,
        maximum_flow::maximum_flow,
);

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
