use graph_library::{algorithms::iter::TraversalType, Graph};
use rstest::rstest;

#[rstest]
#[case("resources/test_graphs/undirected/Graph1.txt", 2)]
#[case("resources/test_graphs/undirected/Graph2.txt", 4)]
#[case("resources/test_graphs/undirected/Graph3.txt", 4)]
#[case("resources/test_graphs/undirected/Graph_gross.txt", 222)]
#[case("resources/test_graphs/undirected/Graph_ganzgross.txt", 9560)]
#[case("resources/test_graphs/undirected/Graph_ganzganzgross.txt", 306)]
fn count_connected_subgraphs(#[case] input_path: &str, #[case] expected_count: u32) {
    let graph = Graph::from_hoever_file(input_path, false)
        .unwrap_or_else(|e| panic!("Graph could not be constructed from file: {:?}", e));

    // Count connected subgraphs
    let count = graph
        .count_connected_subgraphs(Some(TraversalType::BFS))
        .unwrap_or_else(|e| panic!("Failed to count connected subgraphs: {:?}", e));

    // Verify expected count
    assert_eq!(
        count, expected_count,
        "For graph {}, expected {} connected subgraphs, but got {}",
        input_path, expected_count, count
    );
}
