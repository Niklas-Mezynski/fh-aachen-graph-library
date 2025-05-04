use criterion::{black_box, Criterion};
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

/// Create a graph from a file for benchmarking purposes
fn create_test_graph(file: &str) -> MatrixGraph<TestVertex, TestEdge, Undirected> {
    MatrixGraph::<_, _, Undirected>::from_hoever_file(file, TestVertex, |remaining| {
        TestEdge(
            remaining[0]
                .parse()
                .expect("Graph file value must be a float"),
        )
    })
    .unwrap_or_else(|e| panic!("Graph could not be constructed from file: {:?}", e))
}

pub fn tsp(c: &mut Criterion) {
    // Files for exact algorithms (smaller instances)
    let small_files = [
        "resources/test_graphs/complete_undirected_weighted/K_10.txt",
        "resources/test_graphs/complete_undirected_weighted/K_10e.txt",
        "resources/test_graphs/complete_undirected_weighted/K_12.txt",
        "resources/test_graphs/complete_undirected_weighted/K_12e.txt",
    ];

    // Files for heuristic algorithms (larger instances)
    let large_files = [
        "resources/test_graphs/complete_undirected_weighted/K_15.txt",
        "resources/test_graphs/complete_undirected_weighted/K_15e.txt",
        "resources/test_graphs/complete_undirected_weighted/K_20.txt",
        "resources/test_graphs/complete_undirected_weighted/K_30.txt",
        "resources/test_graphs/complete_undirected_weighted/K_50.txt",
        "resources/test_graphs/complete_undirected_weighted/K_70.txt",
        "resources/test_graphs/complete_undirected_weighted/K_100.txt",
    ];

    // Brute-force benchmarks (exact algorithm on small instances)
    {
        let mut group = c.benchmark_group("tsp_brute_force");
        for file in small_files {
            let file_name = std::path::Path::new(file)
                .file_name()
                .unwrap_or_default()
                .to_string_lossy();

            group.bench_function(file_name, |b| {
                let graph = create_test_graph(file);
                b.iter(|| {
                    graph
                        .tsp_brute_force(black_box(None))
                        .unwrap_or_else(|e| panic!("Could not compute TSP: {:?}", e));
                });
            });
        }
        group.finish();
    }

    // TODO: Branch & Bound algorithm benchmarks

    // Nearest Neighbor benchmarks (heuristic algorithm on larger instances)
    {
        let mut group = c.benchmark_group("tsp_nearest_neighbor");
        let all_files = small_files.iter().chain(large_files.iter());

        for file in all_files {
            let file_name = std::path::Path::new(file)
                .file_name()
                .unwrap_or_default()
                .to_string_lossy();

            group.bench_function(file_name, |b| {
                let graph = create_test_graph(file);
                b.iter(|| {
                    graph
                        .tsp_nearest_neighbor(black_box(None))
                        .unwrap_or_else(|e| panic!("Could not compute TSP: {:?}", e));
                });
            });
        }
        group.finish();
    }

    // Double Tree benchmarks (heuristic algorithm on larger instances)
    {
        let mut group = c.benchmark_group("tsp_double_tree");
        let all_files = small_files.iter().chain(large_files.iter());

        for file in all_files {
            let file_name = std::path::Path::new(file)
                .file_name()
                .unwrap_or_default()
                .to_string_lossy();

            group.bench_function(file_name, |b| {
                let graph = create_test_graph(file);
                b.iter(|| {
                    graph
                        .tsp_double_tree(black_box(None))
                        .unwrap_or_else(|e| panic!("Could not compute TSP: {:?}", e));
                });
            });
        }
        group.finish();
    }
}
