use std::fmt::Debug;

use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead};

use rustc_hash::FxHashSet;

use crate::graph::adjacency_list::AdjacencyListGraph;

use super::error::GraphError;
use super::error::ParsingError;
use super::traits::{GraphInterface, WithID};

pub type VertexIDType = u32;

#[derive(Debug, Clone)]
pub struct Vertex {
    pub id: VertexIDType,
}

impl WithID<Vertex, VertexIDType> for Vertex {
    fn get_id(&self) -> VertexIDType {
        self.id
    }
}

#[derive(Debug)]
pub struct Graph<VId = VertexIDType, VertexT = Vertex, Edge = ()>
where
    VId: 'static,
    VertexT: WithID<VertexT, VId> + 'static,
    Edge: 'static,
{
    backend: Box<dyn GraphInterface<VId, VertexT, Edge>>,
}

impl<VId, Vertex: WithID<Vertex, VId>, Edge> Graph<VId, Vertex, Edge>
where
    VId: Debug + Eq + Hash + Copy,
    Vertex: Debug,
    Edge: Debug + Clone,
{
    /// Creates a new graph, from given vertices and edges
    ///
    /// Here I can also make decisions about which graph backend to use
    pub fn from(
        _n_vertices: VertexIDType, // Could be used for pre-allocating memory or hashmap capacity
        vertices: Vec<Vertex>,
        edges: Vec<(VId, VId, Edge)>,
        directed: bool,
    ) -> Result<Self, GraphError<VId>> {
        let mut graph = Graph::<VId, Vertex, Edge> {
            backend: Box::new(AdjacencyListGraph::new()),
        };

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
    Edge: Debug + Clone,
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
        let file = File::open(path).map_err(GraphError::IoError)?;

        // Read the file line by line, and return an iterator of the lines.
        let reader = io::BufReader::new(file);

        let mut n_vertices = None;
        let mut vertices: Vec<Vertex> = vec![];
        let mut vertex_ids: FxHashSet<VertexIDType> = FxHashSet::default();
        let mut edges: Vec<(VertexIDType, VertexIDType, Edge)> = vec![];

        for (line_number, line) in reader.lines().enumerate() {
            let line = line.map_err(GraphError::IoError)?;

            match line_number {
                // Parse first line (Number of vertices)
                0 => {
                    n_vertices = Some(
                        line.parse::<VertexIDType>()
                            .map_err(|e| GraphError::ParseError(ParsingError::Int(e)))?,
                    );

                    if n_vertices.unwrap() == 0 {
                        return Err(GraphError::InvalidFormat(
                            "Number of vertices must be greater than 0".to_string(),
                        ));
                    }
                }
                // Parse edges
                _ => {
                    let mut parsed_line = line.split('\t');

                    let from = parsed_line
                        .next()
                        .ok_or_else(|| {
                            GraphError::InvalidFormat(
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
                    if let Some(n) = n_vertices {
                        if from >= n || to >= n {
                            return Err(GraphError::InvalidFormat(format!(
                                "Vertex ID out of range: expected 0-{}, got {} or {}",
                                n - 1,
                                from,
                                to
                            )));
                        }
                    }

                    let edge = edge_builder(parsed_line.collect::<Vec<&str>>());

                    if !vertex_ids.contains(&from) {
                        vertex_ids.insert(from);
                        vertices.push(Vertex { id: from });
                    }
                    if !vertex_ids.contains(&to) {
                        vertex_ids.insert(to);
                        vertices.push(Vertex { id: to });
                    }

                    edges.push((from, to, edge));
                }
            }
        }

        if n_vertices.is_none() {
            return Err(GraphError::InvalidFormat("Empty file".to_string()));
        }

        if edges.is_empty() {
            return Err(GraphError::InvalidFormat(
                "No edges found in file".to_string(),
            ));
        }

        Graph::from(n_vertices.unwrap(), vertices, edges, directed)
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

impl<VId, Vertex: WithID<Vertex, VId>, Edge> GraphInterface<VId, Vertex, Edge>
    for Graph<VId, Vertex, Edge>
where
    VId: Debug + Eq + Hash,
    Vertex: Debug,
    Edge: Debug + Clone,
{
    fn push_vertex(&mut self, vertex: Vertex) -> Result<(), GraphError<VId>> {
        self.backend.push_vertex(vertex)
    }

    fn push_edge(&mut self, from: VId, to: VId, edge: Edge) -> Result<(), GraphError<VId>> {
        self.backend.push_edge(from, to, edge)
    }

    fn push_undirected_edge(
        &mut self,
        from: VId,
        to: VId,
        edge: Edge,
    ) -> Result<(), GraphError<VId>> {
        self.backend.push_undirected_edge(from, to, edge)
    }

    fn get_all_vertices(&self) -> Vec<&Vertex> {
        self.backend.get_all_vertices()
    }
}
