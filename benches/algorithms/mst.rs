use criterion::{black_box, Criterion};
use graph_library::{
    graph::{EdgeWithWeight, ListGraphBackend, Vertex},
    ListGraph, Undirected,
};

/// Create a graph from a file for benchmarking purposes
fn create_test_graph(file: &str) -> ListGraph<Vertex, EdgeWithWeight, Undirected> {
    ListGraph::<_, _, Undirected>::from_hoever_file_with_weights(file, |remaining| {
        EdgeWithWeight::new(
            remaining[0]
                .parse()
                .expect("Graph file value must be a float"),
        )
    })
    .unwrap_or_else(|e| panic!("Graph could not be constructed from file: {:?}", e))
}

pub fn mst(c: &mut Criterion) {
    let files = [
        "resources/test_graphs/undirected_weighted/G_1_2.txt",
        "resources/test_graphs/undirected_weighted/G_1_20.txt",
        "resources/test_graphs/undirected_weighted/G_1_200.txt",
        "resources/test_graphs/undirected_weighted/G_10_20.txt",
        "resources/test_graphs/undirected_weighted/G_10_200.txt",
        "resources/test_graphs/undirected_weighted/G_100_200.txt",
    ];

    // Prim's algorithm benchmarks
    {
        let mut group = c.benchmark_group("mst_prim");
        for file in &files {
            let file_name = std::path::Path::new(file)
                .file_name()
                .unwrap_or_default()
                .to_string_lossy();

            group.bench_function(file_name, |b| {
                let graph = create_test_graph(file);
                b.iter(|| {
                    graph
                        .mst_prim::<ListGraphBackend<_, _, Undirected>>(black_box(None))
                        .unwrap_or_else(|e| panic!("Could not compute MST: {:?}", e));
                });
            });
        }
        group.finish();
    }

    // Kruskal's algorithm benchmarks
    {
        let mut group = c.benchmark_group("mst_kruskal");
        for file in &files {
            let file_name = std::path::Path::new(file)
                .file_name()
                .unwrap_or_default()
                .to_string_lossy();

            group.bench_function(file_name, |b| {
                let graph = create_test_graph(file);
                b.iter(|| {
                    black_box(
                        graph
                            .mst_kruskal::<ListGraphBackend<_, _, Undirected>>()
                            .unwrap_or_else(|e| panic!("Could not compute MST: {:?}", e)),
                    );
                });
            });
        }
        group.finish();
    }
}
