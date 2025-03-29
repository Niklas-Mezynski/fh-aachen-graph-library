use graph_library::{traits::GraphInterface, Graph};

#[test]
fn create_from_file() {
    let graph = Graph::from_hoever_file("resources/test_graphs/undirected/Graph1.txt", false);
    let vertices = graph.get_all_vertices();
    assert_eq!(vertices.len(), 15);
}
