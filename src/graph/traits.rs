use std::{iter::Sum, ops::Div};

use super::{error::GraphError, Direction};

pub trait WithID {
    type IDType;

    fn get_id(&self) -> Self::IDType;
}

pub trait WeightedEdge {
    type WeightType: Sum + Div<Output = Self::WeightType> + From<u8> + PartialOrd;

    fn get_weight(&self) -> Self::WeightType;
}

pub type EdgeTuple<VId, Edge> = (VId, VId, Edge);
pub trait GraphBase: Default {
    type Vertex: WithID;
    type Edge;
    type Direction: Direction;

    // --- Construction operations ---
    /// Creates a new empty graph of the same backend type.
    fn new() -> Self
    where
        Self: Sized;

    fn new_with_size(n_vertices: usize) -> Self
    where
        Self: Sized;

    fn from_vertices_and_edges(
        vertices: Vec<Self::Vertex>,
        edges: Vec<EdgeTuple<<Self::Vertex as WithID>::IDType, Self::Edge>>,
    ) -> Result<Self, GraphError<<Self::Vertex as WithID>::IDType>>
    where
        Self: Sized;

    // --- Basic Graph operations ---
    /// Adds a new vertex to the graph.
    fn push_vertex(
        &mut self,
        vertex: Self::Vertex,
    ) -> Result<(), GraphError<<Self::Vertex as WithID>::IDType>>;

    /// Adds a new edge between two vertices.
    fn push_edge(
        &mut self,
        from: <Self::Vertex as WithID>::IDType,
        to: <Self::Vertex as WithID>::IDType,
        edge: Self::Edge,
    ) -> Result<(), GraphError<<Self::Vertex as WithID>::IDType>>;

    /// Returns whether the graph is a directed (true) or undirected (false) graph.
    fn is_directed(&self) -> bool;

    // Graph queries

    /// Get vertex data by vertex id.
    fn get_vertex_by_id(
        &self,
        vertex_id: <Self::Vertex as WithID>::IDType,
    ) -> Option<&Self::Vertex>;

    /// Get a mutable reference to vertex data by vertex id.
    fn get_vertex_by_id_mut(
        &mut self,
        vertex_id: <Self::Vertex as WithID>::IDType,
    ) -> Option<&mut Self::Vertex>;

    /// Gets the edge data between two vertices
    fn get_edge(
        &self,
        from_id: <Self::Vertex as WithID>::IDType,
        to_id: <Self::Vertex as WithID>::IDType,
    ) -> Option<&Self::Edge>;

    /// Get all vertices in the graph.
    fn get_all_vertices<'a>(&'a self) -> impl Iterator<Item = &'a Self::Vertex>
    where
        Self::Vertex: 'a;

    /// Get all edges in the graph as an iterator.
    fn get_all_edges<'a>(
        &'a self,
    ) -> impl Iterator<
        Item = (
            <Self::Vertex as WithID>::IDType,
            <Self::Vertex as WithID>::IDType,
            &'a Self::Edge,
        ),
    >
    where
        Self::Edge: 'a;

    /// Get all direct neighbors as an iterator.
    fn get_adjacent_vertices<'a>(
        &'a self,
        vertex_id: <Self::Vertex as WithID>::IDType,
    ) -> impl Iterator<Item = &'a Self::Vertex>
    where
        Self::Vertex: 'a;

    /// Get all direct neighbors including the edge data as an iterator.
    fn get_adjacent_vertices_with_edges<'a>(
        &'a self,
        vertex_id: <Self::Vertex as WithID>::IDType,
    ) -> impl Iterator<Item = (&'a Self::Vertex, &'a Self::Edge)>
    where
        Self::Vertex: 'a,
        Self::Edge: 'a;

    /// Returns the number of vertices in the graph.
    fn vertex_count(&self) -> usize;

    /// Returns the number of edges in the graph.
    fn edge_count(&self) -> usize;

    /// Gets the sum of all edges' weights
    fn get_total_weight(&self) -> <Self::Edge as WeightedEdge>::WeightType
    where
        Self::Edge: WeightedEdge;
}
