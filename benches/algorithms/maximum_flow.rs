use std::hint::black_box;

use criterion::Criterion;
use graph_library::{
    graph::{GraphBase, ListGraph, ListGraphBackend, Vertex},
    Directed,
};

#[derive(Debug, Clone)]
struct FlowEdge {
    max_flow: f64,
    flow: f64,
}

/// Create a directed flow graph from a file for benchmarking purposes
fn create_directed_flow_graph(file: &str) -> ListGraph<Vertex, FlowEdge, Directed> {
    ListGraph::<_, _, Directed>::from_hoever_file_with_weights(file, |remaining| FlowEdge {
        max_flow: remaining[0]
            .parse()
            .expect("Graph file value must be a float"),
        flow: f64::default(),
    })
    .unwrap_or_else(|e| panic!("Graph could not be constructed from file: {:?}", e))
}

pub fn maximum_flow(c: &mut Criterion) {
    let flow_files = [
        "resources/test_graphs/directed_flow/Fluss1.txt",
        "resources/test_graphs/directed_flow/Fluss2.txt",
        "resources/test_graphs/undirected_weighted/G_1_2.txt",
    ];

    let mut group = c.benchmark_group("maximum_flow_edmonds_karp");

    for file in flow_files {
        let file_name = std::path::Path::new(file)
            .file_name()
            .unwrap_or_default()
            .to_string_lossy();

        group.bench_function(file_name, |b| {
            let mut graph = create_directed_flow_graph(file);

            b.iter(|| {
                // Reset flow values before each iteration
                for (_, _, edge) in graph.get_all_edges_mut() {
                    edge.flow = 0.0;
                }

                black_box(
                    graph.edmonds_karp::<ListGraphBackend<_, _, Directed>, _, _, _>(
                        black_box(0),
                        black_box(7),
                        |e| &mut e.flow,
                        |e| &e.max_flow,
                    ),
                )
                .expect("Algorithm should not error");
            });
        });
    }

    group.finish();
}
