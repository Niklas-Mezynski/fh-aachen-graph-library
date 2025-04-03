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

    /// Create a new Graph and tries to preallocate data structures based on the number of vertices/edges
    pub fn new_with_size(
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

    fn push_edge_internal(&mut self, from: VId, to: VId, edge: Edge) -> Result<(), GraphError<VId>>
    where
        VId: Eq + Hash,
    {
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
}

impl<VId, Vertex, Edge> GraphInterface<VId, Vertex, Edge> for AdjacencyListGraph<VId, Vertex, Edge>
where
    VId: Eq + Hash + Copy,
    Vertex: WithID<VId>,
    Edge: Clone,
{
    fn push_vertex(&mut self, vertex: Vertex) -> Result<(), GraphError<VId>> {
        let vid = vertex.get_id();
        if self.vertices.contains_key(&vid) {
            return Err(GraphError::DuplicateVertex(vid));
        }

        self.vertices.insert(vid, vertex);
        Ok(())
    }

    fn push_edge(&mut self, from: VId, to: VId, edge: Edge) -> Result<(), GraphError<VId>> {
        if !self.is_directed {
            return Err(GraphError::DirectedOperationOnUndirectedGraph);
        }

        self.push_edge_internal(from, to, edge)
    }

    fn push_undirected_edge(
        &mut self,
        from: VId,
        to: VId,
        edge: Edge,
    ) -> Result<(), GraphError<VId>> {
        if self.is_directed {
            return Err(GraphError::UndirectedOperationOnDirectedGraph);
        }

        self.push_edge_internal(from, to, edge.clone())?;
        self.push_edge_internal(to, from, edge)?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct MockVertex {
        id: u32,
    }

    impl WithID<u32> for MockVertex {
        fn get_id(&self) -> u32 {
            self.id
        }
    }

    #[test]
    fn test_push_vertex() {
        let mut graph: AdjacencyListGraph<u32, MockVertex, ()> = AdjacencyListGraph::new(true);
        let vertex1 = MockVertex { id: 1 };
        let vertex2 = MockVertex { id: 2 };

        assert!(graph.push_vertex(vertex1).is_ok());
        assert!(graph.push_vertex(vertex2).is_ok());
        assert!(graph.push_vertex(MockVertex { id: 1 }).is_err()); // Duplicate
    }

    #[test]

    fn test_push_edge() {
        let mut graph: AdjacencyListGraph<u32, MockVertex, u32> = AdjacencyListGraph::new(true);
        let vertex1 = MockVertex { id: 1 };
        let vertex2 = MockVertex { id: 2 };

        graph.push_vertex(vertex1).unwrap();
        graph.push_vertex(vertex2).unwrap();

        assert!(graph.push_edge(1, 2, 10).is_ok());
        assert!(graph.push_edge(1, 2, 20).is_err()); // Duplicate edge
        assert!(graph.push_edge(2, 1, 30).is_ok());
        assert!(graph.push_edge(3, 1, 40).is_err()); // Non existent vertex
        assert!(graph.push_edge(1, 3, 40).is_err()); // Non existent vertex
    }

    #[test]
    fn test_push_undirected_edge() {
        let mut graph: AdjacencyListGraph<u32, MockVertex, u32> = AdjacencyListGraph::new(false);
        let vertex1 = MockVertex { id: 1 };
        let vertex2 = MockVertex { id: 2 };

        graph.push_vertex(vertex1).unwrap();
        graph.push_vertex(vertex2).unwrap();

        assert!(graph.push_undirected_edge(1, 2, 10).is_ok());
        assert!(graph.push_undirected_edge(1, 2, 20).is_err()); // Duplicate edge

        let adj_1 = graph.adjacency.get(&1).unwrap();
        assert_eq!(adj_1.len(), 1);
        assert_eq!(adj_1[0].0, 2);
        assert_eq!(adj_1[0].1, 10);

        let adj_2 = graph.adjacency.get(&2).unwrap();
        assert_eq!(adj_2.len(), 1);
        assert_eq!(adj_2[0].0, 1);
        assert_eq!(adj_2[0].1, 10);
    }

    #[test]
    fn test_push_undirected_edge_on_directed_graph() {
        let mut graph: AdjacencyListGraph<u32, MockVertex, u32> = AdjacencyListGraph::new(true);
        let vertex1 = MockVertex { id: 1 };
        let vertex2 = MockVertex { id: 2 };

        graph.push_vertex(vertex1).unwrap();
        graph.push_vertex(vertex2).unwrap();

        assert!(matches!(
            graph.push_undirected_edge(1, 2, 10),
            Err(GraphError::UndirectedOperationOnDirectedGraph)
        ));
    }

    #[test]
    fn test_push_directed_edge_on_undirected_graph() {
        let mut graph: AdjacencyListGraph<u32, MockVertex, u32> = AdjacencyListGraph::new(false);
        let vertex1 = MockVertex { id: 1 };
        let vertex2 = MockVertex { id: 2 };

        graph.push_vertex(vertex1).unwrap();
        graph.push_vertex(vertex2).unwrap();

        assert!(matches!(
            graph.push_edge(1, 2, 10),
            Err(GraphError::DirectedOperationOnUndirectedGraph)
        ));
    }

    #[test]
    fn test_get_vertex() {
        let mut graph: AdjacencyListGraph<u32, MockVertex, ()> = AdjacencyListGraph::new(true);
        let vertex1 = MockVertex { id: 1 };
        let vertex2 = MockVertex { id: 2 };

        graph.push_vertex(vertex1).unwrap();
        graph.push_vertex(vertex2).unwrap();

        let v = graph.get_vertex_by_id(&1).unwrap();
        assert_eq!(v.id, 1);
        let v = graph.get_vertex_by_id(&2).unwrap();
        assert_eq!(v.id, 2);
        assert!(matches!(
            graph.get_vertex_by_id(&3),
            Err(GraphError::VertexNotFound(3))
        ));
    }

    #[test]

    fn test_get_all_vertices() {
        let mut graph: AdjacencyListGraph<u32, MockVertex, ()> = AdjacencyListGraph::new(true);
        let vertex1 = MockVertex { id: 1 };
        let vertex2 = MockVertex { id: 2 };

        graph.push_vertex(vertex1).unwrap();
        graph.push_vertex(vertex2).unwrap();

        let vertices: Vec<_> = graph
            .get_all_vertices()
            .iter()
            .map(|v| v.get_id())
            .collect();
        assert_eq!(vertices.len(), 2);
        assert!(vertices.contains(&1));
        assert!(vertices.contains(&2));
    }

    #[test]

    fn test_get_adjacent_vertices() {
        let mut graph: AdjacencyListGraph<u32, MockVertex, u32> = AdjacencyListGraph::new(true);
        let vertex1 = MockVertex { id: 1 };
        let vertex2 = MockVertex { id: 2 };
        let vertex3 = MockVertex { id: 3 };

        graph.push_vertex(vertex1).unwrap();
        graph.push_vertex(vertex2).unwrap();
        graph.push_vertex(vertex3).unwrap();

        graph.push_edge(1, 2, 10).unwrap();
        graph.push_edge(1, 3, 20).unwrap();

        let adjacent_vertices = graph.get_adjacent_vertices(&1).unwrap();
        assert_eq!(adjacent_vertices.len(), 2);
        assert_eq!(adjacent_vertices[0].id, 2);
        assert_eq!(adjacent_vertices[1].id, 3);

        let adjacent_vertices = graph.get_adjacent_vertices(&2).unwrap();
        assert_eq!(adjacent_vertices.len(), 0);

        assert!(graph.get_adjacent_vertices(&4).is_err());
    }
}
