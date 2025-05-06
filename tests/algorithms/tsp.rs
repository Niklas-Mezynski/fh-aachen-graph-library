use graph_library::graph::{GraphBase, MatrixGraph};
use graph_library::Undirected;
use itertools::Itertools;
use rstest::rstest;

use super::{TestEdge, TestVertex};

/// Enumeration of TSP algorithms for parametrized tests
#[derive(Debug)]
enum TspAlgorithm {
    BruteForce,
    BranchAndBound,
    NearestNeighbor,
    DoubleTree,
}

/// Helper function to create a graph from a file for testing
fn create_test_graph(path: &str) -> MatrixGraph<TestVertex, TestEdge, Undirected> {
    MatrixGraph::<_, _, Undirected>::from_hoever_file(path, TestVertex, |remaining| {
        TestEdge(
            remaining[0]
                .parse()
                .expect("Graph file value must be a float"),
        )
    })
    .unwrap_or_else(|e| panic!("Graph could not be constructed from file: {:?}", e))
}

#[rstest]
#[case("resources/test_graphs/complete_undirected_weighted/K_10.txt", 38.41)]
#[case("resources/test_graphs/complete_undirected_weighted/K_10e.txt", 27.26)]
#[case("resources/test_graphs/complete_undirected_weighted/K_12.txt", 45.19)]
#[case("resources/test_graphs/complete_undirected_weighted/K_12e.txt", 36.13)]
fn tsp_finds_optimal_solution(
    #[case] input_path: &str,
    #[case] expected_optimal_cost: f64,
    #[values(TspAlgorithm::BruteForce, TspAlgorithm::BranchAndBound)] algorithm: TspAlgorithm,
) {
    let graph = create_test_graph(input_path);

    let optimal_path = match algorithm {
        TspAlgorithm::BruteForce => graph.tsp_brute_force(None),
        TspAlgorithm::BranchAndBound => graph.tsp_branch_and_bound(None),
        _ => unreachable!(),
    }
    .unwrap_or_else(|e| panic!("Could not compute tsp solution: {:?}", e));

    let total_cost = optimal_path.total_cost();

    // Verify the path visits all vertices exactly once
    assert_eq!(graph.vertex_count(), optimal_path.len());

    assert_eq!(
        optimal_path
            .edges()
            .map(|(from, _, _)| from)
            .unique()
            .count(),
        graph.vertex_count(),
        "Path should visit each vertex exactly once"
    );
    assert_eq!(
        optimal_path.edges().map(|(_, to, _)| to).unique().count(),
        graph.vertex_count(),
        "Path should visit each vertex exactly once"
    );

    // Verify the cost is within tolerance of expected optimal
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
    #[values(TspAlgorithm::NearestNeighbor, TspAlgorithm::DoubleTree)] algorithm: TspAlgorithm,
) {
    let graph = create_test_graph(input_path);

    let (optimal_path, cost_to_check): (_, Option<f64>) = match algorithm {
        TspAlgorithm::NearestNeighbor => (graph.tsp_nearest_neighbor(None), None),
        TspAlgorithm::DoubleTree => (
            graph.tsp_double_tree(None),
            expected_optimal_cost.map(|v| v * 2_f64),
        ),
        _ => unreachable!(),
    };
    let optimal_path =
        optimal_path.unwrap_or_else(|e| panic!("Could not compute tsp solution: {:?}", e));

    let total_cost = optimal_path.total_cost();

    // Verify the path visits all vertices exactly once
    assert_eq!(graph.vertex_count(), optimal_path.len());

    assert_eq!(
        optimal_path
            .edges()
            .map(|(from, _, _)| from)
            .unique()
            .count(),
        graph.vertex_count(),
        "Path should visit each vertex exactly once"
    );
    assert_eq!(
        optimal_path.edges().map(|(_, to, _)| to).unique().count(),
        graph.vertex_count(),
        "Path should visit each vertex exactly once"
    );

    // Print information for small graph instances only to avoid cluttering test output
    if graph.vertex_count() <= 15 {
        println!("{}", optimal_path);
        println!("Total cost: {}", total_cost);
    }

    // Check against expected costs if provided
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
