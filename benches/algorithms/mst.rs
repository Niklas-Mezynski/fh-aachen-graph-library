use criterion::{BenchmarkId, Criterion};
use graph_library::{graph::EdgeWithWeight, Graph};

pub fn mst(c: &mut Criterion) {
    let files = [
        "resources/test_graphs/undirected_weighted/G_1_2.txt",
        "resources/test_graphs/undirected_weighted/G_1_20.txt",
        "resources/test_graphs/undirected_weighted/G_1_200.txt",
        "resources/test_graphs/undirected_weighted/G_10_20.txt",
        "resources/test_graphs/undirected_weighted/G_10_200.txt",
        "resources/test_graphs/undirected_weighted/G_100_200.txt",
    ];

    let mut group_prim = c.benchmark_group("Build MST (Prim)");

    for file in files {
        let graph = Graph::from_hoever_file_with_weights(file, false, |remaining| {
            EdgeWithWeight::new(
                remaining[0]
                    .parse()
                    .expect("Graph file value must be a float"),
            )
        })
        .unwrap_or_else(|e| panic!("Graph could not be constructed from file: {:?}", e));

        group_prim.bench_function(BenchmarkId::new("build_mst", file), |b| {
            b.iter(|| {
                graph
                    .mst_prim()
                    .unwrap_or_else(|e| panic!("Could not compute mst: {:?}", e));
            });
        });

        // Add same test for group_kruskal
    }
    group_prim.finish();
}
