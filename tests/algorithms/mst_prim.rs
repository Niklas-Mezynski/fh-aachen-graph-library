use graph_library::{algorithms::iter::TraversalType, Graph};
use rstest::rstest;

#[rstest]
#[case("resources/test_graphs/undirected_weighted/G_1_2.txt", 287.32286)]
#[case("resources/test_graphs/undirected_weighted/G_1_20.txt", 36.86275)]
#[case("resources/test_graphs/undirected_weighted/G_1_200.txt", 12.68182)]
#[case("resources/test_graphs/undirected_weighted/G_10_20.txt", 2785.62417)]
#[case("resources/test_graphs/undirected_weighted/G_10_200.txt", 372.14417)]
#[case("resources/test_graphs/undirected_weighted/G_100_200.txt", 27550.51488)]
fn mst_prim(#[case] input_path: &str, #[case] expected_total_weight: f64) {
    todo!()
}
