use graph_library::graph::{MatrixGraph, WeightedEdge, WithID};
use graph_library::Undirected;
use rstest::rstest;

#[derive(Debug)]
enum Algorithms {
    BruteForce,
    BranchNBound,
    NearestNeigbour,
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
fn tsp_exact(
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
        Algorithms::BruteForce => graph.tsp_brute_force(),
        Algorithms::BranchNBound => todo!(),
        _ => unreachable!(),
    }
    .unwrap_or_else(|e| panic!("Could not compute tsp solution: {:?}", e));

    let total_cost = optimal_path.total_cost();

    assert!(
        (total_cost - expected_optimal_cost).abs() < 1e-2,
        "For graph {}, expected optimal TSP cost to be {}, but got {}",
        input_path,
        expected_optimal_cost,
        total_cost
    );
}
