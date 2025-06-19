use std::hint::black_box;

use criterion::Criterion;
use graph_library::{
    graph::{GraphBase, WithID},
    Directed, ListGraph,
};

#[derive(Debug, Clone)]
pub struct BalanceVertex {
    pub id: i32,
    pub balance: i32,
}

impl WithID for BalanceVertex {
    type IDType = i32;

    fn get_id(&self) -> i32 {
        self.id
    }
}

#[derive(Debug, Clone)]
struct CostFlowEdge {
    cost: i32,
    max_flow: i32,
    flow: i32,
}

/// Create a directed min cost flow graph from a file for benchmarking purposes
fn create_min_cost_flow_graph(file: &str) -> ListGraph<BalanceVertex, CostFlowEdge, Directed> {
    ListGraph::<_, _, Directed>::from_hoever_file_with_special_vertices(
        file,
        |index, remaining| BalanceVertex {
            id: index as i32,
            balance: remaining[0]
                .parse::<f32>()
                .expect("Vertex balance value must be an int") as i32,
        },
        |remaining| CostFlowEdge {
            cost: remaining[0]
                .parse::<f32>()
                .expect("Edge cost value must be an int") as i32,
            max_flow: remaining[1]
                .parse::<f32>()
                .expect("Edge max capacity value must be an int") as i32,
            flow: i32::default(),
        },
    )
    .unwrap_or_else(|e| panic!("Graph could not be constructed from file: {:?}", e))
}

pub fn min_cost_flow(c: &mut Criterion) {
    let flow_files = [
        "resources/test_graphs/min_cost_flow/Kostenminimal1.txt",
        "resources/test_graphs/min_cost_flow/Kostenminimal2.txt",
        "resources/test_graphs/min_cost_flow/Kostenminimal_gross1.txt",
        "resources/test_graphs/min_cost_flow/Kostenminimal_gross2.txt",
    ];

    // Cycle Cancelling algorithm benchmarks
    {
        let mut group = c.benchmark_group("min_cost_flow_cycle_cancelling");

        for file in &flow_files {
            let file_name = std::path::Path::new(file)
                .file_name()
                .unwrap_or_default()
                .to_string_lossy();

            group.bench_function(file_name, |b| {
                let mut graph = create_min_cost_flow_graph(file);

                b.iter(|| {
                    // Reset flow values before each iteration
                    for (_, _, edge) in graph.get_all_edges_mut() {
                        edge.flow = 0;
                    }

                    black_box(graph.cycle_cancelling(
                        |v| &v.balance,
                        |e| &e.flow,
                        |e| &mut e.flow,
                        |e| &e.max_flow,
                        |e| &e.cost,
                        [
                            BalanceVertex {
                                id: -1,
                                balance: i32::default(),
                            },
                            BalanceVertex {
                                id: -2,
                                balance: i32::default(),
                            },
                        ],
                        |balance| CostFlowEdge {
                            cost: i32::default(),
                            max_flow: balance,
                            flow: i32::default(),
                        },
                    ))
                    .expect("Algorithm should not error");
                });
            });
        }

        group.finish();
    }

    // Successive Shortest Path algorithm benchmarks
    {
        let mut group = c.benchmark_group("min_cost_flow_successive_shortest_path");

        for file in &flow_files {
            let file_name = std::path::Path::new(file)
                .file_name()
                .unwrap_or_default()
                .to_string_lossy();

            group.bench_function(file_name, |b| {
                let mut graph = create_min_cost_flow_graph(file);

                b.iter(|| {
                    // Reset flow values before each iteration
                    for (_, _, edge) in graph.get_all_edges_mut() {
                        edge.flow = 0;
                    }

                    black_box(graph.successive_shortest_path(
                        |v| &v.balance,
                        |e| &mut e.flow,
                        |e| &e.max_flow,
                        |e| &e.cost,
                    ))
                    .expect("Algorithm should not error");
                });
            });
        }

        group.finish();
    }
}
