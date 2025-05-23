use graph_library::graph::EdgeWithWeight;
use graph_library::{graph::GraphBase, ListGraph, Undirected};
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
    let graph = ListGraph::<_, _, Undirected>::from_hoever_file_default(input_path).unwrap();
    let vertices = graph.get_all_vertices().collect::<Vec<_>>();
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
        ListGraph::<_, _, Undirected>::from_hoever_file_with_weights(input_path, |remaining| {
            EdgeWithWeight::new(
                remaining[0]
                    .parse()
                    .expect("Graph file value must be a float"),
            )
        })
        .unwrap();
    let vertices = graph.get_all_vertices().collect::<Vec<_>>();
    assert_eq!(vertices.len(), expected_vertices);
}
