use criterion::{BenchmarkId, Criterion};
use graph_library::{
    graph::{MatrixGraph, WeightedEdge, WithID},
    Undirected,
};

#[derive(Debug, Clone)]
struct TestVertex(pub usize);

impl WithID for TestVertex {
    type IDType = usize;

    fn get_id(&self) -> Self::IDType {
        self.0
    }
}

#[derive(Debug, Clone)]
struct TestEdge(pub f64);

impl WeightedEdge for TestEdge {
    type WeightType = f64;

    fn get_weight(&self) -> Self::WeightType {
        self.0
    }
}

pub fn tsp(c: &mut Criterion) {
    let files = [
        "resources/test_graphs/complete_undirected_weighted/K_10.txt",
        "resources/test_graphs/complete_undirected_weighted/K_10e.txt",
        "resources/test_graphs/complete_undirected_weighted/K_12.txt",
        "resources/test_graphs/complete_undirected_weighted/K_12e.txt",
    ];

    // Brute-force
    let mut group = c.benchmark_group("Solve TSP (brute-force");
    for file in files {
        group.bench_function(
            BenchmarkId::new("tsp_brute_force", file),
            |b: &mut criterion::Bencher<'_>| {
                let graph = MatrixGraph::<_, _, Undirected>::from_hoever_file(
                    file,
                    TestVertex,
                    |remaining| {
                        TestEdge(
                            remaining[0]
                                .parse()
                                .expect("Graph file value must be a float"),
                        )
                    },
                )
                .unwrap_or_else(|e| panic!("Graph could not be constructed from file: {:?}", e));

                b.iter(|| {
                    graph
                        .tsp_brute_force()
                        .unwrap_or_else(|e| panic!("Could not compute tsp: {:?}", e));
                });
            },
        );
    }
    group.finish();

    // TODO: B&B

    // -------------- All test graphs --------------
    let files = [
        "resources/test_graphs/complete_undirected_weighted/K_15.txt",
        "resources/test_graphs/complete_undirected_weighted/K_15e.txt",
        "resources/test_graphs/complete_undirected_weighted/K_20.txt",
        "resources/test_graphs/complete_undirected_weighted/K_30.txt",
        "resources/test_graphs/complete_undirected_weighted/K_50.txt",
        "resources/test_graphs/complete_undirected_weighted/K_70.txt",
        "resources/test_graphs/complete_undirected_weighted/K_100.txt",
    ];

    // Nearest neighbor
    let mut group = c.benchmark_group("Solve TSP (Nearest Neighbor");
    for file in files {
        group.bench_function(
            BenchmarkId::new("tsp_nearest_neighbor", file),
            |b: &mut criterion::Bencher<'_>| {
                let graph = MatrixGraph::<_, _, Undirected>::from_hoever_file(
                    file,
                    TestVertex,
                    |remaining| {
                        TestEdge(
                            remaining[0]
                                .parse()
                                .expect("Graph file value must be a float"),
                        )
                    },
                )
                .unwrap_or_else(|e| panic!("Graph could not be constructed from file: {:?}", e));

                b.iter(|| {
                    graph
                        .tsp_nearest_neighbor()
                        .unwrap_or_else(|e| panic!("Could not compute tsp: {:?}", e));
                });
            },
        );
    }
    group.finish();
}
