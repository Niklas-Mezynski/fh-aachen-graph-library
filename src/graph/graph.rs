use std::fmt::Debug;
use std::fs::{self};
use std::hash::Hash;

use crate::graph::adjacency_list::AdjacencyListGraph;

use super::error::GraphError;
use super::error::ParsingError;
use super::traits::GraphInterface;
use super::traits::WithID;

pub type VertexIDType = u32;

#[derive(Debug, Clone)]
pub struct Vertex {
    pub id: VertexIDType,
}

impl WithID<VertexIDType> for Vertex {
    fn get_id(&self) -> VertexIDType {
        self.id
    }
}

pub enum GraphBackend {
    AdjacencyList,
}

#[derive(Debug)]
pub struct Graph<VId = VertexIDType, VertexT = Vertex, Edge = ()>
where
    VId: Eq + Hash + Copy + 'static,
    VertexT: WithID<VId> + 'static,
    Edge: 'static,
{
    backend: Box<dyn GraphInterface<VId, VertexT, Edge>>,
}

impl<VId, Vertex, Edge> Graph<VId, Vertex, Edge>
where
    VId: Eq + Hash + Copy + Debug,
    Vertex: WithID<VId> + Debug,
    Edge: Clone + Debug,
{
    /// Creates a new empty graph
    pub fn new(backend_type: GraphBackend) -> Self {
        Graph {
            backend: Box::new(match backend_type {
                GraphBackend::AdjacencyList => AdjacencyListGraph::new(),
            }),
        }
    }

    /// Create a new Graph and tries to preallocate data structures based on the number of vertices/edges
    fn new_with_size(
        backend_type: GraphBackend,
        vertex_count: Option<usize>,
        edge_count: Option<usize>,
    ) -> Self {
        Graph {
            backend: Box::new(match backend_type {
                GraphBackend::AdjacencyList => {
                    AdjacencyListGraph::new_with_size(vertex_count, edge_count)
                }
            }),
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

impl<VId, Vertex, Edge> Default for Graph<VId, Vertex, Edge>
where
    VId: Eq + Hash + Copy + Debug,
    Vertex: WithID<VId> + Debug,
    Edge: Clone + Debug,
{
    fn default() -> Self {
        Self::new(GraphBackend::AdjacencyList)
    }
}

impl<Edge> Graph<VertexIDType, Vertex, Edge>
where
    Edge: Clone + Debug,
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

// Implement the public facing methods directly on Graph
impl<VId, Vertex, Edge> Graph<VId, Vertex, Edge>
where
    VId: Eq + Hash + Copy,
    Vertex: WithID<VId>,
    Edge: Clone,
{
    /// Adds a new vertex to the graph
    ///
    /// See [`GraphInterface::push_vertex`] for details
    pub fn push_vertex(&mut self, vertex: Vertex) -> Result<(), GraphError<VId>> {
        self.backend.push_vertex(vertex)
    }

    /// Adds a new directed edge between two vertices
    ///
    /// See [`GraphInterface::push_edge`] for details
    pub fn push_edge(&mut self, from: VId, to: VId, edge: Edge) -> Result<(), GraphError<VId>> {
        self.backend.push_edge(from, to, edge)
    }

    /// Adds an undirected edge (edges in both directions) between two vertices
    ///
    /// See [`GraphInterface::push_undirected_edge`] for details
    pub fn push_undirected_edge(
        &mut self,
        from: VId,
        to: VId,
        edge: Edge,
    ) -> Result<(), GraphError<VId>> {
        self.backend.push_undirected_edge(from, to, edge)
    }

    /// Gets a vertex by its ID
    ///
    /// See [`GraphInterface::get_vertex_by_id`] for details
    pub fn get_vertex_by_id(&self, vertex_id: &VId) -> Result<&Vertex, GraphError<VId>> {
        self.backend.get_vertex_by_id(vertex_id)
    }

    /// Get all vertices in the graph
    ///
    /// See [`GraphInterface::get_all_vertices`] for details
    pub fn get_all_vertices(&self) -> Vec<&Vertex> {
        self.backend.get_all_vertices()
    }

    /// Get all direct neighbors of a vertex
    ///
    /// See [`GraphInterface::get_adjacent_vertices`] for details
    pub fn get_adjacent_vertices(&self, vertex: &VId) -> Result<Vec<&Vertex>, GraphError<VId>> {
        self.backend.get_adjacent_vertices(vertex)
    }
}
