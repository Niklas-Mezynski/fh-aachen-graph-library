use graph_library::graph::GraphBase;
use graph_library::graph::ListGraphBackend;
use graph_library::Directed;
use graph_library::ListGraph;
use rstest::rstest;

#[derive(Debug, Clone)]
struct FlowEdge {
    max_flow: f64,
    flow: f64,
}

#[rstest]
#[case("resources/test_graphs/directed_flow/Fluss1.txt", 0, 7, 4.0)]
#[case("resources/test_graphs/directed_flow/Fluss2.txt", 0, 7, 5.0)]
#[case("resources/test_graphs/undirected_weighted/G_1_2.txt", 0, 7, 0.75447)]
fn finds_max_flow(
    #[case] input_path: &str,
    #[case] start: u32,
    #[case] target: u32,
    #[case] expected_max_flow: f64,
) {
    let mut graph =
        ListGraph::<_, _, Directed>::from_hoever_file_with_weights(input_path, |remaining| {
            FlowEdge {
                max_flow: remaining[0]
                    .parse()
                    .expect("Graph file value must be a float"),
                flow: f64::default(),
            }
        })
        .unwrap_or_else(|e| panic!("Graph could not be constructed from file: {:?}", e));

    graph
        .edmonds_karp::<ListGraphBackend<_, _, Directed>, _, _, _>(
            start,
            target,
            |e| &mut e.flow,
            |e| &e.max_flow,
        )
        .expect("Error running algorithm");

    // The graph's flow values should be updated now
    // The graph's flow values should be updated now
    let outgoing_flow: f64 = graph
        .get_adjacent_vertices_with_edges(start)
        .map(|(_, edge)| edge.flow)
        .sum();

    let incoming_flow: f64 = graph
        .get_all_edges()
        .filter(|(_, to, _)| to == &target)
        .map(|(_, _, edge)| edge.flow)
        .sum();

    assert!(
        (outgoing_flow - incoming_flow).abs() < 1e-5,
        "Outgoing flow {} does not match incoming flow {}",
        outgoing_flow,
        incoming_flow
    );

    assert!(
        (outgoing_flow - expected_max_flow).abs() < 1e-5,
        "Expected max flow {}, but got {}",
        expected_max_flow,
        outgoing_flow
    );
}
