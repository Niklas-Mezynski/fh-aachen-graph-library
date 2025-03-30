use std::fmt::Debug;

use super::error::GraphError;

pub trait WithID<BaseType, IDType> {
    fn get_id(&self) -> IDType;
}

pub trait GraphInterface<VId, Vertex: WithID<Vertex, VId>, Edge>: Debug {
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
    fn get_all_vertices(&self) -> Vec<&Vertex>;
    // fn has_vertex(&self, vertex: &Vertex) -> bool;
    // fn has_edge(&self, from: &Vertex, to: &Vertex) -> bool;
    // fn neighbors(&self, vertex: &Vertex) -> Vec<&Vertex>;
}
