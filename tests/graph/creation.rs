use graph_library::Graph;
use rstest::rstest;

#[rstest]
#[case("resources/test_graphs/undirected/Graph1.txt", 15)]
#[case("resources/test_graphs/undirected/Graph2.txt", 1000)]
#[case("resources/test_graphs/undirected/Graph3.txt", 1000)]
#[case("resources/test_graphs/undirected/Graph_gross.txt", 100000)]
#[case("resources/test_graphs/undirected/Graph_ganzgross.txt", 500000)]
#[case("resources/test_graphs/undirected/Graph_ganzganzgross.txt", 1000000)]
fn create_from_file_creates_all_vertices(
    #[case] input_path: &str,
    #[case] expected_vertices: usize,
) {
    let graph = Graph::from_hoever_file(input_path, false).unwrap();
    let vertices = graph.get_all_vertices();
    assert_eq!(vertices.len(), expected_vertices);
}

#[rstest]
#[case("resources/test_graphs/undirected_weighted/G_1_2.txt", 1000)]
#[case("resources/test_graphs/undirected_weighted/G_1_20.txt", 1000)]
#[case("resources/test_graphs/undirected_weighted/G_1_200.txt", 1000)]
#[case("resources/test_graphs/undirected_weighted/G_10_20.txt", 10000)]
#[case("resources/test_graphs/undirected_weighted/G_10_200.txt", 10000)]
#[case("resources/test_graphs/undirected_weighted/G_100_200.txt", 100000)]
fn create_from_file_with_weights_creates_all_vertices(
    #[case] input_path: &str,
    #[case] expected_vertices: usize,
) {
    let graph =
        Graph::from_hoever_file_with_weights(input_path, false, |remaining| todo!()).unwrap();
    let vertices = graph.get_all_vertices();
    assert_eq!(vertices.len(), expected_vertices);
}
