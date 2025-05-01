use std::{fmt::Debug, iter::Sum, ops::Div};

use super::error::GraphError;

pub trait WithID {
    type IDType;

    fn get_id(&self) -> Self::IDType;
}

pub trait WeightedEdge {
    type WeightType: Sum + Div<Output = Self::WeightType> + From<u8> + PartialOrd;

    fn get_weight(&self) -> Self::WeightType;
}

pub trait GraphBase<Vertex: WithID, Edge>: Debug + Default {
    // --- Construction operations ---
    /// Creates a new empty graph of the same backend type.
    fn new() -> Self
    where
        Self: Sized;

    fn new_with_size(n_vertices: usize) -> Self
    where
        Self: Sized;

    fn from_vertices_and_edges(
        vertices: Vec<Vertex>,
        edges: Vec<(Vertex::IDType, Vertex::IDType, Edge)>,
    ) -> Result<Self, GraphError<Vertex::IDType>>
    where
        Self: Sized;

    // --- Basic Graph operations ---
    /// Adds a new vertex to the graph.
    fn push_vertex(&mut self, vertex: Vertex) -> Result<(), GraphError<Vertex::IDType>>;

    /// Adds a new edge between two vertices.
    fn push_edge(
        &mut self,
        from: Vertex::IDType,
        to: Vertex::IDType,
        edge: Edge,
    ) -> Result<(), GraphError<Vertex::IDType>>;

    /// Returns whether the graph is a directed (true) or undirected (false) graph.
    fn is_directed(&self) -> bool;

    // Graph queries

    /// Get vertex data by vertex id.
    fn get_vertex_by_id(&self, vertex_id: Vertex::IDType) -> Option<&Vertex>;

    /// Get a mutable reference to vertex data by vertex id.
    fn get_vertex_by_id_mut(&mut self, vertex_id: Vertex::IDType) -> Option<&mut Vertex>;

    /// Get all vertices in the graph.
    fn get_all_vertices<'a>(&'a self) -> impl Iterator<Item = &'a Vertex>
    where
        Vertex: 'a;

    /// Get all edges in the graph as an iterator.
    fn get_all_edges<'a>(
        &'a self,
    ) -> impl Iterator<Item = (Vertex::IDType, Vertex::IDType, &'a Edge)>
    where
        Edge: 'a;

    /// Get all direct neighbors as an iterator.
    fn get_adjacent_vertices<'a>(
        &'a self,
        vertex_id: Vertex::IDType,
    ) -> impl Iterator<Item = &'a Vertex>
    where
        Vertex: 'a;

    /// Get all direct neighbors including the edge data as an iterator.
    fn get_adjacent_vertices_with_edges<'a>(
        &'a self,
        vertex_id: Vertex::IDType,
    ) -> impl Iterator<Item = (&'a Vertex, &'a Edge)>
    where
        Vertex: 'a,
        Edge: 'a;

    /// Returns the number of vertices in the graph.
    fn vertex_count(&self) -> usize;

    /// Returns the number of edges in the graph.
    fn edge_count(&self) -> usize;

    /// Gets the sum of all edges' weights
    fn get_total_weight(&self) -> Edge::WeightType
    where
        Edge: WeightedEdge;
}
