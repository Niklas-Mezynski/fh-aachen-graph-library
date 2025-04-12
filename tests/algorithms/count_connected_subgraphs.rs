use graph_library::{algorithms::iter::TraversalType, Graph};
use rstest::rstest;

#[rstest]
#[case("resources/test_graphs/undirected/Graph1.txt", 2)]
#[case("resources/test_graphs/undirected/Graph2.txt", 4)]
#[case("resources/test_graphs/undirected/Graph3.txt", 4)]
#[case("resources/test_graphs/undirected/Graph_gross.txt", 222)]
#[case("resources/test_graphs/undirected/Graph_ganzgross.txt", 9560)]
#[case("resources/test_graphs/undirected/Graph_ganzganzgross.txt", 306)]
fn count_connected_subgraphs(
    #[case] input_path: &str,
    #[case] expected_count: u32,
    #[values(TraversalType::BFS, TraversalType::DFS)]
    // Add other traversal types as needed
    traversal_type: TraversalType,
) {
    let graph = Graph::from_hoever_file(input_path, false)
        .unwrap_or_else(|e| panic!("Graph could not be constructed from file: {:?}", e));

    // Count connected subgraphs
    let count = graph
        .count_connected_subgraphs(Some(traversal_type))
        .unwrap_or_else(|e| panic!("Failed to count connected subgraphs: {:?}", e));

    // Verify expected count
    assert_eq!(
        count, expected_count,
        "For graph {} using {:?}, expected {} connected subgraphs, but got {}",
        input_path, traversal_type, expected_count, count
    );
}

#[rstest]
#[case("resources/test_graphs/undirected_weighted/G_1_2.txt", 1)]
#[case("resources/test_graphs/undirected_weighted/G_1_20.txt", 1)]
#[case("resources/test_graphs/undirected_weighted/G_1_200.txt", 1)]
#[case("resources/test_graphs/undirected_weighted/G_10_20.txt", 1)]
#[case("resources/test_graphs/undirected_weighted/G_10_200.txt", 1)]
#[case("resources/test_graphs/undirected_weighted/G_100_200.txt", 1)]
fn count_connected_subgraphs_p02_undirected_weights(
    #[case] input_path: &str,
    #[case] expected_count: u32,
    #[values(TraversalType::BFS, TraversalType::DFS)]
    // Add other traversal types as needed
    traversal_type: TraversalType,
) {
    use graph_library::graph::WeightedEdge;

    let graph = Graph::from_hoever_file_with_weights(input_path, false, |remaining| {
        WeightedEdge::new(
            remaining[0]
                .parse()
                .expect("Graph file value must be a float"),
        )
    })
    .unwrap_or_else(|e| panic!("Graph could not be constructed from file: {:?}", e));

    // Count connected subgraphs
    let count = graph
        .count_connected_subgraphs(Some(traversal_type))
        .unwrap_or_else(|e| panic!("Failed to count connected subgraphs: {:?}", e));

    // Verify expected count
    assert_eq!(
        count, expected_count,
        "For graph {} using {:?}, expected {} connected subgraphs, but got {}",
        input_path, traversal_type, expected_count, count
    );
}
