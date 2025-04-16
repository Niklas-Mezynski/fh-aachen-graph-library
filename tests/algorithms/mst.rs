use graph_library::graph::EdgeWithWeight;
use graph_library::Graph;
use rstest::rstest;

#[derive(Debug)]
enum Algorithms {
    Prim,
    Kruskal,
}

#[rstest]
#[case("resources/test_graphs/undirected_weighted/G_1_2.txt", 287.32286)]
#[case("resources/test_graphs/undirected_weighted/G_1_20.txt", 36.86275)]
#[case("resources/test_graphs/undirected_weighted/G_1_200.txt", 12.68182)]
#[case("resources/test_graphs/undirected_weighted/G_10_20.txt", 2785.62417)]
#[case("resources/test_graphs/undirected_weighted/G_10_200.txt", 372.14417)]
#[case("resources/test_graphs/undirected_weighted/G_100_200.txt", 27550.51488)]
fn mst(
    #[case] input_path: &str,
    #[case] expected_mst_weight: f64,
    #[values(Algorithms::Prim, Algorithms::Kruskal)] algorithm: Algorithms,
) {
    let graph = Graph::from_hoever_file_with_weights(input_path, false, |remaining| {
        EdgeWithWeight::new(
            remaining[0]
                .parse()
                .expect("Graph file value must be a float"),
        )
    })
    .unwrap_or_else(|e| panic!("Graph could not be constructed from file: {:?}", e));

    let mst = match algorithm {
        Algorithms::Prim => graph.mst_prim(),
        Algorithms::Kruskal => graph.mst_kruskal(),
    }
    .unwrap_or_else(|e| panic!("Could not compute mst: {:?}", e));

    let total_weight = mst.get_total_weight();

    assert!(
        (total_weight - expected_mst_weight).abs() < 1e-5,
        "For graph {}, expected MST-weight of {}, but got {}",
        input_path,
        expected_mst_weight,
        total_weight
    );
}
