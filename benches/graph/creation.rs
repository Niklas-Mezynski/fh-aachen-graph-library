use criterion::{BenchmarkId, Criterion};
use graph_library::{ListGraph, Undirected};

pub fn graph_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("Graph Creation");

    let files = [
        "resources/test_graphs/undirected/Graph1.txt",
        "resources/test_graphs/undirected/Graph2.txt",
        "resources/test_graphs/undirected/Graph3.txt",
        "resources/test_graphs/undirected/Graph_gross.txt",
        "resources/test_graphs/undirected/Graph_ganzgross.txt",
        "resources/test_graphs/undirected/Graph_ganzganzgross.txt",
    ];

    for file in files {
        group.bench_with_input(BenchmarkId::new("from_file", file), &file, |b, &file| {
            b.iter(|| ListGraph::<_, _, Undirected>::from_hoever_file(file).unwrap());
        });
    }

    group.finish();
}
