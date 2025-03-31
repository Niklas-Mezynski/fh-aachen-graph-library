use algorithms::*;
use criterion::{criterion_group, criterion_main, Criterion};
use graph::*;
mod algorithms;
mod graph;

criterion_main!(benches);
criterion_group!(
        name = benches;
        config = Criterion::default().sample_size(10);
        targets = creation::graph_creation, count_connected_subgraphs::count_connected_subgraphs
);
