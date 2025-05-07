use criterion::{BenchmarkId, Criterion};
use graph_library::{graph::MatrixGraph, ListGraph, Undirected};
use std::{hint::black_box, path::Path};

use crate::{TestEdge, TestVertex};

pub fn graph_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("graph_creation");

    let big_graph_files = [
        "resources/test_graphs/undirected/Graph1.txt",
        "resources/test_graphs/undirected/Graph2.txt",
        "resources/test_graphs/undirected/Graph3.txt",
        "resources/test_graphs/undirected/Graph_gross.txt",
        "resources/test_graphs/undirected/Graph_ganzgross.txt",
        "resources/test_graphs/undirected/Graph_ganzganzgross.txt",
    ];

    // Benchmark ListGraph creation
    for file in &big_graph_files {
        let file_name = Path::new(file)
            .file_name()
            .unwrap_or_default()
            .to_string_lossy();

        group.bench_function(BenchmarkId::new("list_graph", file_name), |b| {
            b.iter(|| {
                ListGraph::<_, _, Undirected>::from_hoever_file_default(black_box(file))
                    .unwrap_or_else(|e| panic!("Failed to create graph: {:?}", e))
            });
        });
    }

    // Benchmark MatrixGraph creation for smaller graphs
    // Matrix representation may be too memory-intensive for very large graphs
    let smaller_graph_files = [
        "resources/test_graphs/complete_undirected_weighted/K_30.txt",
        "resources/test_graphs/complete_undirected_weighted/K_50.txt",
        "resources/test_graphs/complete_undirected_weighted/K_70.txt",
        "resources/test_graphs/complete_undirected_weighted/K_100.txt",
    ];

    for file in smaller_graph_files {
        let file_name = Path::new(file)
            .file_name()
            .unwrap_or_default()
            .to_string_lossy();

        group.bench_function(BenchmarkId::new("matrix_graph", file_name), |b| {
            b.iter(|| {
                MatrixGraph::<TestVertex, TestEdge, Undirected>::from_hoever_file(
                    black_box(file),
                    TestVertex,
                    |remaining| {
                        TestEdge(
                            remaining[0]
                                .parse()
                                .expect("Graph file value must be a float"),
                        )
                    },
                )
                .unwrap_or_else(|e| panic!("Failed to create graph: {:?}", e))
            });
        });
    }

    group.finish();
}
