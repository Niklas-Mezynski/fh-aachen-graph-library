use std::fmt::Debug;

use super::error::GraphError;

pub trait WithID<IDType> {
    fn get_id(&self) -> IDType;
}

pub trait GraphInterface<VId, Vertex: WithID<VId>, Edge>: Debug {
    /// Create a new Graph and tries to preallocate data structures based on the number of vertices/edges
    ///
    /// # Arguments
    /// * `vertex_count`: The expected number of vertices in the graph. This is used to pre-allocate memory for the vertices.
    /// * `edge_count`: The expected number of edges in the graph. This is used to pre-allocate memory for the edges.
    fn new_with_size(
        vertex_count: Option<usize>,
        edge_count: Option<usize>,
        is_directed: bool,
    ) -> Self
    where
        Self: Sized;

    // Basic Graph operations
    /// Adds a new vertex to the graph
    ///
    /// # Errors
    /// - `GraphError::DuplicateVertex`: when trying to add a vertex with an ID that already exists in the graph
    fn push_vertex(&mut self, vertex: Vertex) -> Result<(), GraphError<VId>>;

    /// Adds a new directed edge between two vertices
    ///
    /// # Errors
    /// - `GraphError::VertexNotFound`: when either the source or target vertex ID does not exist
    /// - `GraphError::DuplicateEdge`: when trying to add an edge that already exists
    fn push_edge(&mut self, from: VId, to: VId, edge: Edge) -> Result<(), GraphError<VId>>;

    /// Returns wether the graph is a directed (true) or undirected (false) graph
    fn is_directed(&self) -> bool;

    /// Adds an undirected edge (edges in both directions) between two vertices
    ///
    /// # Errors
    /// - `GraphError::VertexNotFound`: when either the source or target vertex ID does not exist
    /// - `GraphError::DuplicateEdge`: when trying to add an edge that already exists
    fn push_undirected_edge(
        &mut self,
        from: VId,
        to: VId,
        edge: Edge,
    ) -> Result<(), GraphError<VId>>;

    // Graph queries
    fn get_vertex_by_id(&self, vertex_id: &VId) -> Result<&Vertex, GraphError<VId>>;

    fn get_vertex_by_id_mut(&mut self, id: &VId) -> Result<&mut Vertex, GraphError<VId>>;

    /// Get all vertices in the graph
    fn get_all_vertices(&self) -> Vec<&Vertex>;
    /// Get All direct neighbors
    ///
    /// # Errors
    /// - `GraphError::VertexNotFound`: when the vertex does not exist
    fn get_adjacent_vertices(&self, vertex: &VId) -> Result<Vec<&Vertex>, GraphError<VId>>;
    // fn has_vertex(&self, vertex: &Vertex) -> bool;
    // fn has_edge(&self, from: &Vertex, to: &Vertex) -> bool;
}
