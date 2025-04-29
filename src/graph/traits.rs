use std::{iter::Sum, ops::Div};

use super::error::GraphError;

pub trait WithID<IDType> {
    fn get_id(&self) -> IDType;
}

pub trait GraphInterface<VId, Vertex: WithID<VId>, Edge> {
    // --- Basic Graph operations ---
    /// Adds a new vertex to the graph.
    ///
    /// See [`Graph`] for detailed documentation.
    fn push_vertex(&mut self, vertex: Vertex) -> Result<(), GraphError<VId>>;

    /// Adds a new edge between two vertices.
    ///
    /// See [`Graph`] for detailed documentation.
    fn push_edge(&mut self, from: VId, to: VId, edge: Edge) -> Result<(), GraphError<VId>>;

    /// Returns whether the graph is a directed (true) or undirected (false) graph.
    ///
    /// See [`Graph`] for detailed documentation.
    fn is_directed(&self) -> bool;

    // Graph queries

    /// Get vertex data by vertex id.
    ///
    /// See [`Graph`] for detailed documentation.
    fn get_vertex_by_id(&self, vertex_id: &VId) -> Option<&Vertex>;

    /// Get a mutable reference to vertex data by vertex id.
    ///
    /// See [`Graph`] for detailed documentation.
    fn get_vertex_by_id_mut(&mut self, id: &VId) -> Option<&mut Vertex>;

    /// Get all vertices in the graph.
    ///
    /// See [`Graph`] for detailed documentation.
    fn get_all_vertices<'a>(&'a self) -> impl Iterator<Item = &'a Vertex>
    where
        Vertex: 'a;

    /// Get all edges in the graph.
    ///
    /// See [`Graph`] for detailed documentation.
    fn get_all_edges(&self) -> Vec<(&VId, &VId, &Edge)>;

    /// Get all direct neighbors.
    ///
    /// See [`Graph`] for detailed documentation.
    fn get_adjacent_vertices(&self, vertex: &VId) -> Vec<&Vertex>;

    /// Get all direct neighbors including the edge data.
    ///
    /// See [`Graph`] for detailed documentation.
    fn get_adjacent_vertices_with_edges(&self, vertex: &VId) -> Vec<(&Vertex, &Edge)>;

    /// Returns the number of vertices in the graph.
    fn vertex_count(&self) -> usize;

    /// Returns the number of edges in the graph.
    fn edge_count(&self) -> usize;
}

pub trait WeightedEdge {
    type WeightType: Sum + Div<Output = Self::WeightType> + From<u8> + PartialOrd;

    fn get_weight(&self) -> Self::WeightType;
}

pub trait WeightedGraphInterface<VId, Vertex: WithID<VId>, Edge>
where
    Edge: WeightedEdge,
{
    /// Gets the sum of all edges' weights
    fn get_total_weight(&self) -> Edge::WeightType;
}
