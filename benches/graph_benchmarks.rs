use criterion::{criterion_group, criterion_main, Criterion};
mod graph;

criterion_main!(benches);
criterion_group!(
        name = benches;
        config = Criterion::default().sample_size(10);
        targets = graph::creation::bench_graph_creation
);
