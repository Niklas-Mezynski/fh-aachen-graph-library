use graph_library::graph::GraphBase;
use graph_library::graph::WithID;
use graph_library::Directed;
use graph_library::GraphError;
use graph_library::ListGraph;
use rstest::rstest;

#[derive(Debug)]
enum Algorithms {
    CycleCancelling,
    SuccessiveShortestPath,
}

#[derive(Debug, Clone)]
pub struct BalanceVertex {
    pub id: i32,
    pub balance: f32,
}

impl WithID for BalanceVertex {
    type IDType = i32;

    fn get_id(&self) -> i32 {
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
            id: index as i32,
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

    let result = match algorithm {
        Algorithms::CycleCancelling => graph.cycle_cancelling(
            |v| &v.balance,
            |e| &e.flow,
            |e| &mut e.flow,
            |e| &e.max_flow,
            |e| &e.cost,
            [
                BalanceVertex {
                    id: -1,
                    balance: 0.0,
                },
                BalanceVertex {
                    id: -2,
                    balance: 0.0,
                },
            ],
            |balance| CostFlowEdge {
                cost: 0.0,
                max_flow: balance,
                flow: 0.0,
            },
        ),
        Algorithms::SuccessiveShortestPath => todo!(),
    };

    match expected_cost {
        Some(expected_cost) => {
            assert!(result.is_ok());

            // Assert all balances are fulfilled
            for v in graph.get_all_vertices() {
                let outgoing_flow: f32 = graph
                    .get_adjacent_vertices_with_edges(v.get_id())
                    .map(|(_, edge)| edge.flow)
                    .sum();

                let incoming_flow: f32 = graph
                    .get_all_edges()
                    .filter(|(_, to, _)| to == &v.get_id())
                    .map(|(_, _, edge)| edge.flow)
                    .sum();

                assert_eq!(v.balance, outgoing_flow - incoming_flow);
            }

            // Assert costs
            let total_cost: f32 = graph
                .get_all_edges()
                .map(|(_from, _to, edge)| edge.cost * edge.flow)
                .sum();

            assert_eq!(expected_cost as f32, total_cost);
        }
        None => {
            assert!(result.is_err());
            assert!(matches!(result.unwrap_err(), GraphError::AlgorithmError(_)))
        }
    }

    assert!(true);
}
