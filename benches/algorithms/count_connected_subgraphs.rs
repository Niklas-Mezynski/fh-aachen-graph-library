use criterion::{BenchmarkId, Criterion};
use graph_library::{algorithms::iter::TraversalType, Graph};

pub fn count_connected_subgraphs(c: &mut Criterion) {
    let files = [
        "resources/test_graphs/undirected/Graph1.txt",
        "resources/test_graphs/undirected/Graph2.txt",
        "resources/test_graphs/undirected/Graph3.txt",
        "resources/test_graphs/undirected/Graph_gross.txt",
        "resources/test_graphs/undirected/Graph_ganzgross.txt",
        "resources/test_graphs/undirected/Graph_ganzganzgross.txt",
    ];

    let traversals = [
        TraversalType::BFS,
        TraversalType::DFS,
        TraversalType::DFSRecursive,
    ];

    for traversal_type in traversals {
        let mut group =
            c.benchmark_group(format!("Count Connected Subgraphs ({})", traversal_type));

        for file in files {
            let graph = Graph::from_hoever_file(file, false).unwrap();
            group.bench_function(
                BenchmarkId::new(format!("count_connected_{:?}", traversal_type), file),
                |b| {
                    b.iter(|| {
                        graph
                            .count_connected_subgraphs(Some(traversal_type))
                            .unwrap()
                    });
                },
            );
        }
        group.finish();
    }
}
