use std::hint::black_box;

use criterion::Criterion;
use graph_library::{
    graph::{EdgeWithWeight, IntoDirected, ListGraph, Vertex},
    Directed, Undirected,
};

/// Create a directed graph from a file for benchmarking purposes
fn create_directed_graph(file: &str) -> ListGraph<Vertex, EdgeWithWeight, Directed> {
    ListGraph::<_, _, Directed>::from_hoever_file_with_weights(file, |remaining| {
        EdgeWithWeight::new(
            remaining[0]
                .parse()
                .expect("Graph file value must be a float"),
        )
    })
    .unwrap_or_else(|e| panic!("Graph could not be constructed from file: {:?}", e))
}

/// Create an undirected graph from a file and convert to directed in order to run the algorithm
fn create_undirected_graph_as_directed(file: &str) -> ListGraph<Vertex, EdgeWithWeight, Directed> {
    ListGraph::<_, _, Undirected>::from_hoever_file_with_weights(file, |remaining| {
        EdgeWithWeight::new(
            remaining[0]
                .parse()
                .expect("Graph file value must be a float"),
        )
    })
    .unwrap_or_else(|e| panic!("Graph could not be constructed from file: {:?}", e))
    .into_directed()
}

pub fn shortest_path(c: &mut Criterion) {
    let directed_positive_files = [
        "resources/test_graphs/directed_weighted/Wege1.txt",
        "resources/test_graphs/undirected_weighted/G_1_2.txt",
        "resources/test_graphs/undirected_weighted/G_1_20.txt",
        "resources/test_graphs/undirected_weighted/G_10_20.txt",
    ];
    let directed_negative_files = [
        "resources/test_graphs/directed_weighted/Wege2.txt",
        "resources/test_graphs/directed_weighted/Wege3.txt",
    ];

    let undirected_positive_files = [
        "resources/test_graphs/undirected_weighted/G_1_2.txt",
        "resources/test_graphs/undirected_weighted/G_1_20.txt",
        "resources/test_graphs/undirected_weighted/G_10_20.txt",
    ];

    // Dijkstra benchmarks
    {
        let mut group = c.benchmark_group("shortest_path_dijkstra");
        for file in directed_positive_files {
            let file_name = std::path::Path::new(file)
                .file_name()
                .unwrap_or_default()
                .to_string_lossy();
            group.bench_function(file_name, |b| {
                let graph = create_directed_graph(file);
                b.iter(|| {
                    black_box(graph.dijkstra(black_box(0), None));
                });
            });
        }
        for file in &undirected_positive_files {
            let file_name = std::path::Path::new(file)
                .file_name()
                .unwrap_or_default()
                .to_string_lossy();
            group.bench_function(format!("{file_name} (undirected)"), |b| {
                let graph = create_undirected_graph_as_directed(file);
                b.iter(|| {
                    black_box(graph.dijkstra(black_box(0), None));
                });
            });
        }
        group.finish();
    }

    // Bellman-Ford benchmarks (directed)
    {
        let mut group = c.benchmark_group("shortest_path_bellman_ford");
        for file in directed_positive_files
            .iter()
            .chain(directed_negative_files.iter())
        {
            let file_name = std::path::Path::new(file)
                .file_name()
                .unwrap_or_default()
                .to_string_lossy();
            group.bench_function(file_name, |b| {
                let graph = create_directed_graph(file);
                b.iter(|| {
                    black_box(graph.bellman_ford(black_box(0)));
                });
            });
        }
        for file in &undirected_positive_files {
            let file_name = std::path::Path::new(file)
                .file_name()
                .unwrap_or_default()
                .to_string_lossy();
            group.bench_function(format!("{file_name} (undirected)"), |b| {
                let graph = create_undirected_graph_as_directed(file);
                b.iter(|| {
                    black_box(graph.bellman_ford(black_box(0)));
                });
            });
        }
        group.finish();
    }
}
