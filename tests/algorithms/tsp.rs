use graph_library::graph::{GraphBase, MatrixGraph, WeightedEdge, WithID};
use graph_library::Undirected;
use rstest::rstest;

#[derive(Debug)]
enum Algorithms {
    BruteForce,
    BranchNBound,
    NearestNeighbor,
    DoubleTree,
}

#[derive(Debug, Clone)]
struct TestVertex(pub usize);

impl WithID for TestVertex {
    type IDType = usize;

    fn get_id(&self) -> Self::IDType {
        self.0
    }
}

#[derive(Debug, Clone)]
struct TestEdge(pub f64);

impl WeightedEdge for TestEdge {
    type WeightType = f64;

    fn get_weight(&self) -> Self::WeightType {
        self.0
    }
}

#[rstest]
#[case("resources/test_graphs/complete_undirected_weighted/K_10.txt", 38.41)]
#[case("resources/test_graphs/complete_undirected_weighted/K_10e.txt", 27.26)]
#[case("resources/test_graphs/complete_undirected_weighted/K_12.txt", 45.19)]
#[case("resources/test_graphs/complete_undirected_weighted/K_12e.txt", 36.13)]
fn tsp_finds_optimal_solution(
    #[case] input_path: &str,
    #[case] expected_optimal_cost: f64,
    #[values(Algorithms::BruteForce)] algorithm: Algorithms,
) {
    let graph =
        MatrixGraph::<_, _, Undirected>::from_hoever_file(input_path, TestVertex, |remaining| {
            TestEdge(
                remaining[0]
                    .parse()
                    .expect("Graph file value must be a float"),
            )
        })
        .unwrap_or_else(|e| panic!("Graph could not be constructed from file: {:?}", e));

    let optimal_path = match algorithm {
        Algorithms::BruteForce => graph.tsp_brute_force(None),
        Algorithms::BranchNBound => todo!(),
        _ => unreachable!(),
    }
    .unwrap_or_else(|e| panic!("Could not compute tsp solution: {:?}", e));

    let total_cost = optimal_path.total_cost();

    assert_eq!(graph.vertex_count(), optimal_path.edges.len());

    assert!(
        (total_cost - expected_optimal_cost).abs() < 1e-2,
        "For graph {}, expected optimal TSP cost to be {}, but got {}",
        input_path,
        expected_optimal_cost,
        total_cost
    );
}

#[rstest]
#[case(
    "resources/test_graphs/complete_undirected_weighted/K_10.txt",
    Some(38.41)
)]
#[case(
    "resources/test_graphs/complete_undirected_weighted/K_10e.txt",
    Some(27.26)
)]
#[case(
    "resources/test_graphs/complete_undirected_weighted/K_12.txt",
    Some(45.19)
)]
#[case(
    "resources/test_graphs/complete_undirected_weighted/K_12e.txt",
    Some(36.13)
)]
#[case("resources/test_graphs/complete_undirected_weighted/K_15.txt", None)]
#[case("resources/test_graphs/complete_undirected_weighted/K_15e.txt", None)]
#[case("resources/test_graphs/complete_undirected_weighted/K_20.txt", None)]
#[case("resources/test_graphs/complete_undirected_weighted/K_30.txt", None)]
#[case("resources/test_graphs/complete_undirected_weighted/K_50.txt", None)]
#[case("resources/test_graphs/complete_undirected_weighted/K_70.txt", None)]
#[case("resources/test_graphs/complete_undirected_weighted/K_100.txt", None)]
fn tsp_finds_solution(
    #[case] input_path: &str,
    #[case] expected_optimal_cost: Option<f64>,
    #[values(Algorithms::NearestNeighbor, Algorithms::DoubleTree)] algorithm: Algorithms,
) {
    let graph =
        MatrixGraph::<_, _, Undirected>::from_hoever_file(input_path, TestVertex, |remaining| {
            TestEdge(
                remaining[0]
                    .parse()
                    .expect("Graph file value must be a float"),
            )
        })
        .unwrap_or_else(|e| panic!("Graph could not be constructed from file: {:?}", e));

    let (optimal_path, cost_to_check): (_, Option<f64>) = match algorithm {
        Algorithms::NearestNeighbor => (graph.tsp_nearest_neighbor(None), None),
        Algorithms::DoubleTree => (
            graph.tsp_double_tree(None),
            expected_optimal_cost.map(|v| v * 2_f64),
        ),
        _ => unreachable!(),
    };
    let optimal_path =
        optimal_path.unwrap_or_else(|e| panic!("Could not compute tsp solution: {:?}", e));

    let total_cost = optimal_path.total_cost();

    assert_eq!(graph.vertex_count(), optimal_path.edges.len());

    if graph.vertex_count() <= 15 {
        println!("{}", optimal_path);
        println!("Total cost: {}", total_cost);
    }

    if let Some(expected_optimal_cost) = cost_to_check {
        assert!(
            total_cost <= expected_optimal_cost + 1e-2,
            "For graph {}, expected TSP cost to be at most {}, but got {}",
            input_path,
            expected_optimal_cost,
            total_cost
        )
    }
}
