use graph_library::graph::WithID;
use graph_library::Directed;
use graph_library::ListGraph;
use rstest::rstest;

#[derive(Debug)]
enum Algorithms {
    CycleCancelling,
    SuccessiveShortestPath,
}

#[derive(Debug, Clone)]
pub struct BalanceVertex {
    pub id: u32,
    pub balance: f32,
}

impl WithID for BalanceVertex {
    type IDType = u32;

    fn get_id(&self) -> u32 {
        self.id
    }
}

#[derive(Debug, Clone)]
struct CostFlowEdge {
    cost: f32,
    max_flow: f32,
    flow: f32,
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
fn finds_min_cost_flow(
    #[case] input_path: &str,
    #[case] expected_cost: Option<i32>,
    #[values(Algorithms::CycleCancelling)] algorithm: Algorithms,
) {
    let mut graph = ListGraph::<_, _, Directed>::from_hoever_file_with_special_vertices(
        input_path,
        |index, remaining| BalanceVertex {
            id: index as u32,
            balance: remaining[0]
                .parse()
                .expect("Vertex balance value must be a float"),
        },
        |remaining| CostFlowEdge {
            cost: remaining[0]
                .parse()
                .expect("Edge cost value must be a float"),
            max_flow: remaining[1]
                .parse()
                .expect("Edge max capacity value must be a float"),
            flow: f32::default(),
        },
    )
    .unwrap_or_else(|e| panic!("Graph could not be constructed from file: {:?}", e));

    assert!(false);
}
