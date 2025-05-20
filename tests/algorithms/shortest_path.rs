use graph_library::{graph::EdgeWithWeight, ListGraph};
use graph_library::{Directed, Undirected};
use rstest::rstest;

#[derive(Debug)]
enum Algorithms {
    Dijkstra,
    BellmanFord,
}

#[rstest]
#[case("resources/test_graphs/directed_weighted/Wege1.txt", 2, 0, 6.0)]
#[case("resources/test_graphs/undirected_weighted/G_1_2.txt", 0, 1, 5.56283)]
fn shortest_path_directed_positive_weights(
    #[case] input_path: &str,
    #[case] from: u32,
    #[case] to: u32,
    #[case] expected_shortest_path_length: f64,
    #[values(Algorithms::Dijkstra, Algorithms::BellmanFord)] algorithm: Algorithms,
) {
    let graph =
        ListGraph::<_, _, Directed>::from_hoever_file_with_weights(input_path, |remaining| {
            EdgeWithWeight::new(
                remaining[0]
                    .parse()
                    .expect("Graph file value must be a float"),
            )
        })
        .unwrap_or_else(|e| panic!("Graph could not be constructed from file: {:?}", e));

    let (costs, _predecessor) = match algorithm {
        Algorithms::Dijkstra => graph.dijkstra(from, None),
        Algorithms::BellmanFord => todo!(),
    };

    let cost = costs
        .get(&to)
        .unwrap_or_else(|| panic!("Shortest path from {} to {} not found in graph", from, to));

    assert!(
        (cost - expected_shortest_path_length).abs() < 1e-5,
        "For graph {}, expected shortest path from {} to {} to be {}, but got {}",
        input_path,
        from,
        to,
        expected_shortest_path_length,
        cost
    );
}

#[rstest]
#[case("resources/test_graphs/directed_weighted/Wege1.txt", 2, 0, 6.0)]
#[case("resources/test_graphs/undirected_weighted/G_1_2.txt", 0, 1, 5.56283)]
fn shortest_path_directed_positive_weights_early_abort(
    #[case] input_path: &str,
    #[case] from: u32,
    #[case] to: u32,
    #[case] expected_shortest_path_length: f64,
    #[values(Algorithms::Dijkstra, Algorithms::BellmanFord)] algorithm: Algorithms,
) {
    let graph =
        ListGraph::<_, _, Directed>::from_hoever_file_with_weights(input_path, |remaining| {
            EdgeWithWeight::new(
                remaining[0]
                    .parse()
                    .expect("Graph file value must be a float"),
            )
        })
        .unwrap_or_else(|e| panic!("Graph could not be constructed from file: {:?}", e));

    let (costs, _predecessor) = match algorithm {
        Algorithms::Dijkstra => graph.dijkstra(from, Some(to)),
        Algorithms::BellmanFord => todo!(),
    };

    let cost = costs
        .get(&to)
        .unwrap_or_else(|| panic!("Shortest path from {} to {} not found in graph", from, to));

    assert!(
        (cost - expected_shortest_path_length).abs() < 1e-5,
        "For graph {}, expected shortest path from {} to {} to be {}, but got {}",
        input_path,
        from,
        to,
        expected_shortest_path_length,
        cost
    );
}

#[rstest]
#[case("resources/test_graphs/undirected_weighted/G_1_2.txt", 0, 1, 2.36802)]
fn shortest_path_undirected(
    #[case] input_path: &str,
    #[case] from: u32,
    #[case] to: u32,
    #[case] expected_shortest_path_length: f64,
    #[values(Algorithms::Dijkstra, Algorithms::BellmanFord)] algorithm: Algorithms,
) {
    let graph =
        ListGraph::<_, _, Undirected>::from_hoever_file_with_weights(input_path, |remaining| {
            EdgeWithWeight::new(
                remaining[0]
                    .parse()
                    .expect("Graph file value must be a float"),
            )
        })
        .unwrap_or_else(|e| panic!("Graph could not be constructed from file: {:?}", e));

    let (costs, _predecessor) = match algorithm {
        Algorithms::Dijkstra => graph.dijkstra(from, None),
        Algorithms::BellmanFord => todo!(),
    };

    let cost = costs
        .get(&to)
        .unwrap_or_else(|| panic!("Shortest path from {} to {} not found in graph", from, to));

    assert!(
        (cost - expected_shortest_path_length).abs() < 1e-5,
        "For graph {}, expected shortest path from {} to {} to be {}, but got {}",
        input_path,
        from,
        to,
        expected_shortest_path_length,
        cost
    );
}

#[rstest]
#[case("resources/test_graphs/directed_weighted/Wege2.txt", 2, 0, Some(2.0))]
#[case("resources/test_graphs/directed_weighted/Wege3.txt", 2, 0, None)]
fn shortest_path_directed_negative_weights(
    #[case] input_path: &str,
    #[case] from: u32,
    #[case] to: u32,
    #[case] expected_shortest_path_length: Option<f64>,
    #[values(Algorithms::BellmanFord)] algorithm: Algorithms,
) {
    let graph =
        ListGraph::<_, _, Directed>::from_hoever_file_with_weights(input_path, |remaining| {
            EdgeWithWeight::new(
                remaining[0]
                    .parse()
                    .expect("Graph file value must be a float"),
            )
        })
        .unwrap_or_else(|e| panic!("Graph could not be constructed from file: {:?}", e));

    let (costs, _predecessor) = match algorithm {
        Algorithms::Dijkstra => graph.dijkstra(from, None),
        Algorithms::BellmanFord => todo!(),
    };

    let cost = costs
        .get(&to)
        .unwrap_or_else(|| panic!("Shortest path from {} to {} not found in graph", from, to));

    match expected_shortest_path_length {
        Some(expected) => assert!(
            (cost - expected).abs() < 1e-5,
            "For graph {}, expected shortest path from {} to {} to be {}, but got {}",
            input_path,
            from,
            to,
            expected,
            cost
        ),
        None => todo!(),
    }
}
