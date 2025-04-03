use std::{fmt::Debug, hash::Hash};

use rustc_hash::{FxBuildHasher, FxHashMap};

use super::{
    error::GraphError,
    traits::{GraphInterface, WithID},
};

#[derive(Debug)]
pub struct AdjacencyListGraph<VId, Vertex: WithID<VId>, Edge> {
    vertices: FxHashMap<VId, Vertex>,
    adjacency: FxHashMap<VId, Vec<(VId, Edge)>>,
    is_directed: bool,
}

impl<VId, Vertex: WithID<VId>, Edge> AdjacencyListGraph<VId, Vertex, Edge> {
    /// Create a new, empty Graph with an Adjacency List representation
    pub fn new(is_directed: bool) -> Self {
        AdjacencyListGraph {
            vertices: FxHashMap::default(),
            adjacency: FxHashMap::default(),
            is_directed,
        }
    }
}

impl<VId, Vertex: WithID<VId>, Edge> GraphInterface<VId, Vertex, Edge>
    for AdjacencyListGraph<VId, Vertex, Edge>
where
    VId: Debug + Eq + Hash + Copy,
    Vertex: Debug,
    Edge: Debug + Clone,
{
    fn new_with_size(
        vertex_count: Option<usize>,
        _edge_count: Option<usize>,
        is_directed: bool,
    ) -> Self
    where
        Self: Sized,
    {
        AdjacencyListGraph {
            vertices: match vertex_count {
                Some(n_vertices) => FxHashMap::with_capacity_and_hasher(n_vertices, FxBuildHasher),
                None => FxHashMap::default(),
            },
            adjacency: match vertex_count {
                // Should I really allocate one an adjacency list for each vertex?
                // -> Depends on how many lonely vertices I have. Usually there should be no lonely vertices in most cases so I won't optimize for that
                Some(n_vertices) => FxHashMap::with_capacity_and_hasher(n_vertices, FxBuildHasher),
                None => FxHashMap::default(),
            },
            is_directed,
        }
    }

    fn push_vertex(&mut self, vertex: Vertex) -> Result<(), GraphError<VId>> {
        let vid = vertex.get_id();
        if self.vertices.contains_key(&vid) {
            return Err(GraphError::DuplicateVertex(vid));
        }

        self.vertices.insert(vid, vertex);
        Ok(())
    }

    fn push_edge(&mut self, from: VId, to: VId, edge: Edge) -> Result<(), GraphError<VId>> {
        // Check that vertices exist
        if !self.vertices.contains_key(&from) {
            return Err(GraphError::VertexNotFound(from));
        }
        if !self.vertices.contains_key(&to) {
            return Err(GraphError::VertexNotFound(to));
        }

        // Check that edge does not exist yet
        if let Some(e) = self.adjacency.get(&from) {
            if e.iter().any(|(t, _)| t == &to) {
                return Err(GraphError::DuplicateEdge(from, to));
            }
        }

        let curr_adjacency_list = self.adjacency.entry(from).or_default();
        curr_adjacency_list.push((to, edge));
        Ok(())
    }

    fn push_undirected_edge(
        &mut self,
        from: VId,
        to: VId,
        edge: Edge,
    ) -> Result<(), GraphError<VId>> {
        self.push_edge(from, to, edge.clone())?;
        self.push_edge(to, from, edge)?;
        Ok(())
    }

    fn is_directed(&self) -> bool {
        self.is_directed
    }

    fn get_vertex_by_id(&self, vertex_id: &VId) -> Result<&Vertex, GraphError<VId>> {
        self.vertices
            .get(vertex_id)
            .ok_or(GraphError::VertexNotFound(*vertex_id))
    }

    fn get_vertex_by_id_mut(&mut self, id: &VId) -> Result<&mut Vertex, GraphError<VId>> {
        self.vertices
            .get_mut(id)
            .ok_or(GraphError::VertexNotFound(*id))
    }

    fn get_all_vertices(&self) -> Vec<&Vertex> {
        self.vertices.values().collect()
    }

    fn get_adjacent_vertices(&self, vertex: &VId) -> Result<Vec<&Vertex>, GraphError<VId>> {
        if !self.vertices.contains_key(vertex) {
            return Err(GraphError::VertexNotFound(*vertex));
        }

        Ok(self
            .adjacency
            .get(vertex)
            .map(|edges| {
                edges
                    .iter()
                    .map(|(to_id, _)| {
                        self.vertices
                            .get(to_id)
                            .expect("All edges must connect to existing vertices")
                    })
                    .collect()
            })
            .unwrap_or_default())
    }
}
