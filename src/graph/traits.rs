pub trait WithID<BaseType, IDType> {
    fn get_id(&self) -> IDType;
}

pub trait Graph<Vertices = (), Edges = ()> {
    // Erstellung

    /// Creates a new, empty graph.
    fn new() -> Self;

    /// Creates a new graph, from given vertices and edges
    fn from(n_nodes: usize, vertices: Vec<Vertices>, edges: Vec<Edges>, directed: bool) -> Self;

    /// Creates a new graph from a file provided by Prof. Hoever for testing the algorithms.
    ///
    /// # Arguments
    ///
    /// * `path`: The path to the Hoever file.
    fn from_hoever_file(path: &str) -> Self
    where
        Self: std::marker::Sized,
    {
        // TODO: Read file
        Graph::new()
    }

    // Grundlegende Graphenoperationen
    fn add_node(&mut self, node: Vertices);
    fn remove_node(&mut self, node: &Vertices);
    fn add_edge(&mut self, from: &Vertices, to: &Vertices, weight: Option<Edges>);
    fn remove_edge(&mut self, from: &Vertices, to: &Vertices);

    // Abfragemethoden
    fn has_node(&self, node: &Vertices) -> bool;
    fn has_edge(&self, from: &Vertices, to: &Vertices) -> bool;
    fn neighbors(&self, node: &Vertices) -> Vec<&Vertices>;
}
