use graph_library::{Graph, GraphInterface};

#[test]
fn create_from_file() {
    let graph =
        Graph::from_hoever_file("resources/test_graphs/undirected/Graph1.txt", false).unwrap();
    let vertices = graph.get_all_vertices();
    assert_eq!(vertices.len(), 15);
}
