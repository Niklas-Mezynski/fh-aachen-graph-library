use graph_library::{Graph, GraphInterface};
use rstest::rstest;
use std::time::Instant;

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
#[case("resources/test_graphs/undirected/Graph1.txt", 100)]
#[case("resources/test_graphs/undirected/Graph2.txt", 100)]
#[case("resources/test_graphs/undirected/Graph3.txt", 100)]
#[case("resources/test_graphs/undirected/Graph_gross.txt", 1000)]
#[case("resources/test_graphs/undirected/Graph_ganzgross.txt", 1000)]
#[case("resources/test_graphs/undirected/Graph_ganzganzgross.txt", 1000)]
fn creation_performance(#[case] input_path: &str, #[case] max_time_millis: u32) {
    let now = Instant::now();
    let _graph = Graph::from_hoever_file(input_path, false).unwrap();
    let elapsed = now.elapsed();
    assert!(
        elapsed.as_millis() <= max_time_millis.into(),
        "The creation took {}ms, but should be less than {}ms",
        elapsed.as_millis(),
        max_time_millis
    );
}
