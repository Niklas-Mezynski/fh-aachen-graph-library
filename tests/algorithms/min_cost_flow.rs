use graph_library::graph::GraphBase;
use graph_library::Directed;
use graph_library::ListGraph;
use rstest::rstest;

#[derive(Debug, Clone)]
struct FlowEdge {
    max_flow: f64,
    flow: f64,
}

#[rstest]
#[case("resources/test_graphs/min_cost_flow/Kostenminimal1.txt", Some(3))]
#[case("resources/test_graphs/min_cost_flow/Kostenminimal2.txt", Some(0))]
#[case("resources/test_graphs/min_cost_flow/Kostenminimal3.txt", None)]
#[case("resources/test_graphs/min_cost_flow/Kostenminimal4.txt", None)]
#[case(
    "resources/test_graphs/min_cost_flow/Kostenminimal_gross1.txt",
    Some(1537)
)]
#[case(
    "resources/test_graphs/min_cost_flow/Kostenminimal_gross2.txt",
    Some(1838)
)]
#[case("resources/test_graphs/min_cost_flow/Kostenminimal_gross3.txt", None)]
fn finds_min_cost_flow(#[case] input_path: &str, #[case] expected_cost: Option<i32>) {
    // let mut graph =
    //     ListGraph::<_, _, Directed>::from_hoever_file_with_weights(input_path, |remaining| {
    //         FlowEdge {
    //             max_flow: remaining[0]
    //                 .parse()
    //                 .expect("Graph file value must be a float"),
    //             flow: f64::default(),
    //         }
    //     })
    //     .unwrap_or_else(|e| panic!("Graph could not be constructed from file: {:?}", e));

    assert!(false);
}
