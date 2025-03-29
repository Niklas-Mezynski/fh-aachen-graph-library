use std::{fmt::Debug, hash::Hash};

use rustc_hash::FxHashMap;

use super::traits::{GraphInterface, WithID};

#[derive(Debug)]
pub struct AdjacencyListGraph<VId, Vertex: WithID<Vertex, VId>, Edge> {
    vertices: FxHashMap<VId, Vertex>,
    adjacency: FxHashMap<VId, Vec<(VId, Edge)>>,
}

impl<VId, Vertex: WithID<Vertex, VId>, Edge> AdjacencyListGraph<VId, Vertex, Edge> {
    /// Create a new, empty Graph with an Adjacency List representation
    pub fn new() -> Self {
        AdjacencyListGraph {
            vertices: FxHashMap::default(),
            adjacency: FxHashMap::default(),
        }
    }
}

impl<VId, Vertex: WithID<Vertex, VId>, Edge> GraphInterface<VId, Vertex, Edge>
    for AdjacencyListGraph<VId, Vertex, Edge>
where
    VId: Debug + Eq + Hash,
    Vertex: Debug,
    Edge: Debug + Clone,
{
    fn push_vertex(&mut self, vertex: Vertex) {
        self.vertices.insert(vertex.get_id(), vertex);
    }

    fn push_edge(&mut self, from: &Vertex, to: &Vertex, edge: Edge) {
        let curr_adjacency_list = self.adjacency.entry(from.get_id()).or_default();
        curr_adjacency_list.push((to.get_id(), edge));
    }

    fn push_undirected_edge(&mut self, from: &Vertex, to: &Vertex, edge: Edge) {
        self.push_edge(from, to, edge.clone());
        self.push_edge(to, from, edge);
    }

    fn get_all_vertices(&self) -> Vec<&Vertex> {
        self.vertices.values().collect()
    }
}
