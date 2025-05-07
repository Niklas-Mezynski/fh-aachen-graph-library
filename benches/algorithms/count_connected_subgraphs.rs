use criterion::Criterion;
use graph_library::{algorithms::iter::TraversalType, graph::Vertex, ListGraph, Undirected};
use std::hint::black_box;

/// Create a graph from a file for benchmarking purposes
fn create_test_graph(file: &str) -> ListGraph<Vertex, (), Undirected> {
    ListGraph::<_, _, Undirected>::from_hoever_file_default(file)
        .unwrap_or_else(|e| panic!("Graph could not be constructed from file: {:?}", e))
}

pub fn count_connected_subgraphs(c: &mut Criterion) {
    let files = [
        "resources/test_graphs/undirected/Graph1.txt",
        "resources/test_graphs/undirected/Graph2.txt",
        "resources/test_graphs/undirected/Graph3.txt",
        "resources/test_graphs/undirected/Graph_gross.txt",
        "resources/test_graphs/undirected/Graph_ganzgross.txt",
        "resources/test_graphs/undirected/Graph_ganzganzgross.txt",
    ];

    let traversals = [TraversalType::BFS, TraversalType::DFS];

    for &traversal_type in &traversals {
        let group_name = format!("count_connected_subgraphs_{}", traversal_type);
        let mut group = c.benchmark_group(group_name);

        for file in &files {
            let file_name = std::path::Path::new(file)
                .file_name()
                .unwrap_or_default()
                .to_string_lossy();

            group.bench_function(file_name, |b| {
                let graph = create_test_graph(file);
                b.iter(|| {
                    graph
                        .count_connected_subgraphs(black_box(Some(traversal_type)))
                        .unwrap_or_else(|e| panic!("Could not count connected components: {:?}", e))
                });
            });
        }
        group.finish();
    }
}
