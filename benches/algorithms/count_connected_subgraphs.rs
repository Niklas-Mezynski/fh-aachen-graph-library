use criterion::{BenchmarkId, Criterion};
use graph_library::Graph;

pub fn count_connected_subgraphs(c: &mut Criterion) {
    let mut group = c.benchmark_group("Count Connected Subgraphs");

    let files = [
        "resources/test_graphs/undirected/Graph1.txt",
        "resources/test_graphs/undirected/Graph2.txt",
        "resources/test_graphs/undirected/Graph3.txt",
        "resources/test_graphs/undirected/Graph_gross.txt",
        "resources/test_graphs/undirected/Graph_ganzgross.txt",
        "resources/test_graphs/undirected/Graph_ganzganzgross.txt",
    ];

    for file in files {
        let graph = Graph::from_hoever_file(file, false).unwrap();
        group.bench_function(BenchmarkId::new("count_connected", file), |b| {
            b.iter(|| graph.count_connected_subgraphs().unwrap());
        });
    }

    group.finish();
}
