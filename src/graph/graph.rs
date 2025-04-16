use std::fmt::Debug;
use std::fs::{self};
use std::hash::Hash;

use crate::graph::adjacency_list::AdjacencyListGraph;

use super::error::GraphError;
use super::error::ParsingError;
use super::traits::GraphInterface;
use super::traits::WithID;
use super::{Vertex, VertexIDType, WeightedEdge, WeightedGraphInterface};

#[derive(Debug)]
pub enum GraphBackend {
    AdjacencyList,
}

#[derive(Debug)]
enum Backend<VId, Vertex: WithID<VId>, Edge> {
    AdjacencyList(AdjacencyListGraph<VId, Vertex, Edge>),
}

#[derive(Debug)]
pub struct Graph<VId = VertexIDType, VertexT = Vertex, Edge = ()>
where
    VId: Eq + Hash + Copy,
    VertexT: WithID<VId>,
    Edge:,
{
    backend: Backend<VId, VertexT, Edge>,
}

impl<VId, Vertex, Edge> Graph<VId, Vertex, Edge>
where
    VId: Eq + Hash + Copy,
    Vertex: WithID<VId>,
    Edge: Clone,
{
    /// Creates a new empty graph with a default backend
    pub fn new(is_directed: bool) -> Self {
        Graph {
            backend: Backend::AdjacencyList(AdjacencyListGraph::new(is_directed)),
        }
    }

    /// Creates a new empty graph with a given Backend
    pub fn new_with_backend(backend_type: GraphBackend, is_directed: bool) -> Self {
        Graph {
            backend: match backend_type {
                GraphBackend::AdjacencyList => {
                    Backend::AdjacencyList(AdjacencyListGraph::new(is_directed))
                }
            },
        }
    }

    /// Create a new Graph and tries to preallocate data structures based on the number of vertices/edges
    ///
    /// # Arguments
    /// * `backend_type`: Which data representation backend to use
    /// * `vertex_count`: The expected number of vertices in the graph. This is used to pre-allocate memory for the vertices.
    /// * `edge_count`: The expected number of edges in the graph. This is used to pre-allocate memory for the edges.
    /// * `is_directed`: Boolean in indicating wether the graph is directed or not
    fn new_with_size(
        backend_type: GraphBackend,
        vertex_count: Option<usize>,
        edge_count: Option<usize>,
        is_directed: bool,
    ) -> Self {
        Graph {
            backend: match backend_type {
                GraphBackend::AdjacencyList => Backend::AdjacencyList(
                    AdjacencyListGraph::new_with_size(vertex_count, edge_count, is_directed),
                ),
            },
        }
    }

    /// Creates a new graph, from given vertices and edges
    ///
    /// Here I can also make decisions about which graph backend to use
    pub fn from(
        n_vertices: VertexIDType, // Could be used for pre-allocating memory or hashmap capacity
        vertices: Vec<Vertex>,
        edges: Vec<(VId, VId, Edge)>,
        directed: bool,
    ) -> Result<Self, GraphError<VId>> {
        let mut graph = Graph::<VId, Vertex, Edge>::new_with_size(
            GraphBackend::AdjacencyList,
            Some(n_vertices as usize),
            Some(edges.len()),
            directed,
        );

        vertices
            .into_iter()
            .try_for_each(|v| graph.backend.push_vertex(v))?;

        edges
            .into_iter()
            .try_for_each(|(from, to, edge)| match directed {
                true => graph.backend.push_edge(from, to, edge),
                false => graph.backend.push_undirected_edge(from, to, edge),
            })?;

        Ok(graph)
    }
}

impl<Edge> Graph<VertexIDType, Vertex, Edge>
where
    Edge: Clone,
{
    /// Creates a new graph from a file provided by Prof. Hoever for testing the algorithms.
    ///
    /// Format:
    /// - Erste Zeile: Knotenanzahl
    /// - Folgende Zeilen: Kanten (i->j, Nummerierung: 0 ... Knotenanzahl-1)
    pub fn from_hoever_file_with_weights(
        path: &str,
        directed: bool,
        edge_builder: fn(remaining: Vec<&str>) -> Edge,
    ) -> Result<Self, GraphError<VertexIDType>> {
        // Open the file in read-only mode.
        let file_contents = fs::read_to_string(path).map_err(GraphError::IoError)?;
        let mut line_iter = file_contents.lines();

        // Parse first line (number of vertices)
        let n_vertices = line_iter
            .next()
            .ok_or_else(|| {
                GraphError::InvalidFormat(
                    "The file must contain at least one line with the number of edges".to_string(),
                )
            })?
            .parse::<VertexIDType>()
            .map_err(|e| GraphError::ParseError(ParsingError::Int(e)))?;

        if n_vertices == 0 {
            return Err(GraphError::InvalidFormat(
                "Number of vertices must be greater than 0".to_string(),
            ));
        }

        let edges = line_iter
            .map(|line| {
                let mut parsed_line = line.split('\t');

                let from = parsed_line
                    .next()
                    .ok_or_else(|| {
                        GraphError::<VertexIDType>::InvalidFormat(
                            "Missing 'from' vertex id in edge definition".to_string(),
                        )
                    })?
                    .parse::<VertexIDType>()
                    .map_err(|e| GraphError::ParseError(ParsingError::Int(e)))?;

                let to = parsed_line
                    .next()
                    .ok_or_else(|| {
                        GraphError::InvalidFormat(
                            "Missing 'to' vertex id in edge definition".to_string(),
                        )
                    })?
                    .parse::<VertexIDType>()
                    .map_err(|e| GraphError::ParseError(ParsingError::Int(e)))?;

                // Check if vertex IDs are within valid range
                if from >= n_vertices || to >= n_vertices {
                    return Err(GraphError::InvalidFormat(format!(
                        "Vertex ID out of range: expected 0-{}, got {} or {}",
                        n_vertices - 1,
                        from,
                        to
                    )));
                }

                let edge = edge_builder(parsed_line.collect::<Vec<&str>>());

                Ok((from, to, edge))
            })
            .collect::<Result<Vec<_>, GraphError<VertexIDType>>>()?;

        // We create a vertex each for the number of vertices in line 1 (starting at 0)
        let vertices: Vec<Vertex> = (0..n_vertices).map(|vid| Vertex { id: vid }).collect();

        if edges.is_empty() {
            return Err(GraphError::InvalidFormat(
                "No edges found in file".to_string(),
            ));
        }

        Graph::from(n_vertices, vertices, edges, directed)
    }
}

impl Graph<VertexIDType, Vertex, ()> {
    /// Creates a new graph from a file provided by Prof. Hoever for testing the algorithms.
    ///
    /// Format:
    /// - Erste Zeile: Knotenanzahl
    /// - Folgende Zeilen: Kanten (i->j, Nummerierung: 0 ... Knotenanzahl-1)
    pub fn from_hoever_file(path: &str, directed: bool) -> Result<Self, GraphError<VertexIDType>> {
        Graph::from_hoever_file_with_weights(path, directed, |_| ())
    }
}

// --- Implement the public facing methods directly on Graph ---
impl<VId, Vertex, Edge> Graph<VId, Vertex, Edge>
where
    VId: Eq + Hash + Copy,
    Vertex: WithID<VId>,
    Edge: Clone,
{
    /// Adds a new vertex to the graph.
    ///
    /// # Errors
    /// - `GraphError::DuplicateVertex`: when trying to add a vertex with an ID that already exists in the graph
    pub fn push_vertex(&mut self, vertex: Vertex) -> Result<(), GraphError<VId>> {
        self.backend.push_vertex(vertex)
    }

    /// Adds a new directed edge between two vertices.
    ///
    /// # Errors
    /// - `GraphError::VertexNotFound`: when either the source or target vertex ID does not exist
    /// - `GraphError::DuplicateEdge`: when trying to add an edge that already exists
    /// - `GraphError::DirectedOperationOnUndirectedGraph`: when using on an undirected graph
    pub fn push_edge(&mut self, from: VId, to: VId, edge: Edge) -> Result<(), GraphError<VId>> {
        self.backend.push_edge(from, to, edge)
    }

    /// Returns whether the graph is directed (true) or undirected (false).
    pub fn is_directed(&self) -> bool {
        self.backend.is_directed()
    }

    /// Adds an undirected edge (edges in both directions) between two vertices.
    ///
    /// # Errors
    /// - `GraphError::VertexNotFound`: when either the source or target vertex ID does not exist
    /// - `GraphError::DuplicateEdge`: when trying to add an edge that already exists
    /// - `GraphError::UndirectedOperationOnDirectedGraph`: when using on a directed graph
    pub fn push_undirected_edge(
        &mut self,
        from: VId,
        to: VId,
        edge: Edge,
    ) -> Result<(), GraphError<VId>> {
        self.backend.push_undirected_edge(from, to, edge)
    }

    /// Gets a vertex by its ID.
    ///
    /// Returns a reference to the vertex data for the given vertex ID.
    ///
    /// # Errors
    /// - `GraphError::VertexNotFound`: when the vertex does not exist
    pub fn get_vertex_by_id(&self, vertex_id: &VId) -> Result<&Vertex, GraphError<VId>> {
        self.backend.get_vertex_by_id(vertex_id)
    }

    /// Gets a mutable reference to a vertex by its ID.
    ///
    /// Returns a mutable reference to the vertex data for the given vertex ID.
    ///
    /// # Errors
    /// - `GraphError::VertexNotFound`: when the vertex does not exist
    pub fn get_vertex_by_id_mut(
        &mut self,
        vertex_id: &VId,
    ) -> Result<&mut Vertex, GraphError<VId>> {
        self.backend.get_vertex_by_id_mut(vertex_id)
    }

    /// Get all vertices in the graph.
    ///
    /// Returns a vector of references to all vertices in the graph.
    pub fn get_all_vertices(&self) -> Vec<&Vertex> {
        self.backend.get_all_vertices()
    }

    /// Get all direct neighbors of a vertex.
    ///
    /// Returns a vector of references to all vertices directly connected to the given vertex.
    ///
    /// # Errors
    /// - `GraphError::VertexNotFound`: when the vertex does not exist
    pub fn get_adjacent_vertices(&self, vertex: &VId) -> Result<Vec<&Vertex>, GraphError<VId>> {
        self.backend.get_adjacent_vertices(vertex)
    }

    /// Get all direct neighbors of a vertex, with the corresponding edge.
    ///
    /// Returns a vector of tuples containing references to the neighbor vertex and the edge data.
    ///
    /// # Errors
    /// - `GraphError::VertexNotFound`: when the vertex does not exist
    pub fn get_adjacent_vertices_with_edges(
        &self,
        vertex: &VId,
    ) -> Result<Vec<(&Vertex, &Edge)>, GraphError<VId>> {
        self.backend.get_adjacent_vertices_with_edges(vertex)
    }

    /// Get all edges in the graph.
    ///
    /// Returns a vector of tuples containing references to the source vertex ID, target vertex ID, and edge data.
    ///
    /// ## Warning
    /// In undirected graphs, edges may be contained twice, once for each direction.
    pub fn get_all_edges(&self) -> Vec<(&VId, &VId, &Edge)> {
        self.backend.get_all_edges()
    }
}

// Implement the graph backend
impl<VId, Vertex, Edge> GraphInterface<VId, Vertex, Edge> for Backend<VId, Vertex, Edge>
where
    VId: Eq + Hash + Copy,
    Vertex: WithID<VId>,
    Edge: Clone,
{
    fn push_vertex(&mut self, vertex: Vertex) -> Result<(), GraphError<VId>> {
        match self {
            Backend::AdjacencyList(graph) => graph.push_vertex(vertex),
        }
    }

    fn push_edge(&mut self, from: VId, to: VId, edge: Edge) -> Result<(), GraphError<VId>> {
        match self {
            Backend::AdjacencyList(graph) => graph.push_edge(from, to, edge),
        }
    }

    fn is_directed(&self) -> bool {
        match self {
            Backend::AdjacencyList(graph) => graph.is_directed(),
        }
    }

    fn push_undirected_edge(
        &mut self,
        from: VId,
        to: VId,
        edge: Edge,
    ) -> Result<(), GraphError<VId>> {
        match self {
            Backend::AdjacencyList(graph) => graph.push_undirected_edge(from, to, edge),
        }
    }

    fn get_vertex_by_id(&self, vertex_id: &VId) -> Result<&Vertex, GraphError<VId>> {
        match self {
            Backend::AdjacencyList(graph) => graph.get_vertex_by_id(vertex_id),
        }
    }

    fn get_vertex_by_id_mut(&mut self, id: &VId) -> Result<&mut Vertex, GraphError<VId>> {
        match self {
            Backend::AdjacencyList(graph) => graph.get_vertex_by_id_mut(id),
        }
    }

    fn get_all_vertices(&self) -> Vec<&Vertex> {
        match self {
            Backend::AdjacencyList(graph) => graph.get_all_vertices(),
        }
    }

    fn get_adjacent_vertices(&self, vertex: &VId) -> Result<Vec<&Vertex>, GraphError<VId>> {
        match self {
            Backend::AdjacencyList(graph) => graph.get_adjacent_vertices(vertex),
        }
    }

    fn get_adjacent_vertices_with_edges(
        &self,
        vertex: &VId,
    ) -> Result<Vec<(&Vertex, &Edge)>, GraphError<VId>> {
        match self {
            Backend::AdjacencyList(graph) => graph.get_adjacent_vertices_with_edges(vertex),
        }
    }

    fn get_all_edges(&self) -> Vec<(&VId, &VId, &Edge)> {
        match self {
            Backend::AdjacencyList(graph) => graph.get_all_edges(),
        }
    }
}

impl<VId, Vertex, Edge> Graph<VId, Vertex, Edge>
where
    VId: Eq + Hash + Copy,
    Vertex: WithID<VId>,
    Edge: WeightedEdge + Clone,
{
    /// Get sum of all edges' weight
    ///
    /// See [`WeightedGraphInterface::get_total_weight`] for details
    pub fn get_total_weight(&self) -> <Edge as WeightedEdge>::WeightType {
        self.backend.get_total_weight()
    }
}

impl<VId, Vertex, Edge> WeightedGraphInterface<VId, Vertex, Edge> for Backend<VId, Vertex, Edge>
where
    VId: Eq + Hash + Copy,
    Vertex: WithID<VId>,
    Edge: WeightedEdge + Clone,
{
    fn get_total_weight(&self) -> <Edge as WeightedEdge>::WeightType {
        match self {
            Backend::AdjacencyList(graph) => graph.get_total_weight(),
        }
    }
}
