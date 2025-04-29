use std::{fmt::Debug, hash::Hash};

use rustc_hash::{FxBuildHasher, FxHashMap};

use super::{
    error::GraphError,
    traits::{GraphInterface, WithID},
    WeightedEdge, WeightedGraphInterface,
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
    VId: Eq + Hash + PartialOrd + Copy,
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
        if self.is_directed {
            self.push_edge_internal(from, to, edge)?;
        } else {
            self.push_edge_internal(from, to, edge.clone())?;
            self.push_edge_internal(to, from, edge)?;
        }
        Ok(())
    }

    fn is_directed(&self) -> bool {
        self.is_directed
    }

    fn get_vertex_by_id(&self, vertex_id: &VId) -> Option<&Vertex> {
        self.vertices.get(vertex_id)
    }

    fn get_vertex_by_id_mut(&mut self, id: &VId) -> Option<&mut Vertex> {
        self.vertices.get_mut(id)
    }

    fn get_all_vertices<'a>(&'a self) -> impl Iterator<Item = &'a Vertex>
    where
        Vertex: 'a,
    {
        self.vertices.values()
    }

    fn get_adjacent_vertices<'a>(&'a self, vertex: &VId) -> impl Iterator<Item = &'a Vertex>
    where
        Vertex: 'a,
    {
        self.adjacency
            .get(vertex)
            .map(|edges| {
                edges.iter().map(|(to_id, _)| {
                    self.vertices
                        .get(to_id)
                        .expect("All edges must connect to existing vertices")
                })
            })
            .into_iter()
            .flatten()
    }

    fn get_adjacent_vertices_with_edges<'a>(
        &'a self,
        vertex: &VId,
    ) -> impl Iterator<Item = (&'a Vertex, &'a Edge)>
    where
        Vertex: 'a,
        Edge: 'a,
    {
        self.adjacency
            .get(vertex)
            .map(|edges| {
                edges.iter().map(|(to_id, edge)| {
                    (
                        self.vertices
                            .get(to_id)
                            .expect("All edges must connect to existing vertices"),
                        edge,
                    )
                })
            })
            .into_iter()
            .flatten()
    }

    fn get_all_edges<'a>(&'a self) -> Box<dyn Iterator<Item = (&'a VId, &'a VId, &'a Edge)> + 'a>
    where
        VId: 'a,
        Edge: 'a,
    {
        if self.is_directed {
            Box::new(self.adjacency.iter().flat_map(|(from_id, adjacency_list)| {
                adjacency_list
                    .iter()
                    .map(move |(to_id, edge)| (from_id, to_id, edge))
            }))
        } else {
            // For undirected graphs, only return (from, to) where from <= to (assuming VId: Ord)
            Box::new(self.adjacency.iter().flat_map(|(from_id, adjacency_list)| {
                adjacency_list.iter().filter_map(move |(to_id, edge)| {
                    if from_id <= to_id {
                        Some((from_id, to_id, edge))
                    } else {
                        None
                    }
                })
            }))
        }
    }

    fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    fn edge_count(&self) -> usize {
        let edge_count: usize = self.adjacency.values().map(|adj| adj.len()).sum();
        if self.is_directed {
            edge_count
        } else {
            edge_count / 2
        }
    }
}

impl<VId, Vertex, Edge> WeightedGraphInterface<VId, Vertex, Edge>
    for AdjacencyListGraph<VId, Vertex, Edge>
where
    VId: Eq + Hash + Copy,
    Vertex: WithID<VId>,
    Edge: WeightedEdge + Clone,
{
    fn get_total_weight(&self) -> Edge::WeightType {
        let sum = self
            .adjacency
            .values()
            .map(|adjacency_list| {
                adjacency_list
                    .iter()
                    .map(|(_, edge)| edge.get_weight())
                    .sum()
            })
            .sum();

        match self.is_directed {
            true => sum,
            false => sum / Edge::WeightType::from(2),
        }
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
        assert!(matches!(
            graph.push_vertex(MockVertex { id: 1 }),
            Err(GraphError::DuplicateVertex(1))
        )); // Duplicate
    }

    #[test]

    fn test_push_edge() {
        let mut graph: AdjacencyListGraph<u32, MockVertex, u32> = AdjacencyListGraph::new(true);
        let vertex1 = MockVertex { id: 1 };
        let vertex2 = MockVertex { id: 2 };

        graph.push_vertex(vertex1).unwrap();
        graph.push_vertex(vertex2).unwrap();

        assert!(graph.push_edge(1, 2, 10).is_ok());
        assert!(graph.push_edge(2, 1, 30).is_ok());

        assert!(matches!(
            graph.push_edge(1, 2, 20),
            Err(GraphError::DuplicateEdge(1, 2))
        )); // Duplicate edge
        assert!(matches!(
            graph.push_edge(3, 1, 40),
            Err(GraphError::VertexNotFound(3))
        )); // Non existent vertex
        assert!(matches!(
            graph.push_edge(1, 3, 40),
            Err(GraphError::VertexNotFound(3))
        )); // Non existent vertex
    }

    #[test]
    fn test_push_undirected_edge() {
        let mut graph: AdjacencyListGraph<u32, MockVertex, u32> = AdjacencyListGraph::new(false);
        let vertex1 = MockVertex { id: 1 };
        let vertex2 = MockVertex { id: 2 };

        graph.push_vertex(vertex1).unwrap();
        graph.push_vertex(vertex2).unwrap();

        assert!(graph.push_edge(1, 2, 10).is_ok());
        assert!(matches!(
            graph.push_edge(1, 2, 20),
            Err(GraphError::DuplicateEdge(1, 2))
        )); // Duplicate edge

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
        assert!(graph.get_vertex_by_id(&3).is_none());
    }

    #[test]

    fn test_get_all_vertices() {
        let mut graph: AdjacencyListGraph<u32, MockVertex, ()> = AdjacencyListGraph::new(true);
        let vertex1 = MockVertex { id: 1 };
        let vertex2 = MockVertex { id: 2 };

        graph.push_vertex(vertex1).unwrap();
        graph.push_vertex(vertex2).unwrap();

        let vertices: Vec<_> = graph.get_all_vertices().map(|v| v.get_id()).collect();
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

        let adjacent_vertices = graph.get_adjacent_vertices(&1).collect::<Vec<_>>();
        assert_eq!(adjacent_vertices.len(), 2);
        assert_eq!(adjacent_vertices[0].id, 2);
        assert_eq!(adjacent_vertices[1].id, 3);

        let adjacent_vertices = graph.get_adjacent_vertices(&2).collect::<Vec<_>>();
        assert_eq!(adjacent_vertices.len(), 0);

        assert_eq!(graph.get_adjacent_vertices(&4).collect::<Vec<_>>().len(), 0);
    }

    #[test]
    fn test_get_adjacent_vertices_with_edges() {
        let mut graph: AdjacencyListGraph<u32, MockVertex, u32> = AdjacencyListGraph::new(true);
        let vertex1 = MockVertex { id: 1 };
        let vertex2 = MockVertex { id: 2 };
        let vertex3 = MockVertex { id: 3 };

        graph.push_vertex(vertex1).unwrap();
        graph.push_vertex(vertex2).unwrap();
        graph.push_vertex(vertex3).unwrap();

        graph.push_edge(1, 2, 10).unwrap();
        graph.push_edge(1, 3, 20).unwrap();

        let adjacent_vertices = graph
            .get_adjacent_vertices_with_edges(&1)
            .collect::<Vec<_>>();
        assert_eq!(adjacent_vertices.len(), 2);
        assert_eq!(adjacent_vertices[0].0.id, 2);
        assert_eq!(adjacent_vertices[0].1, &10);

        assert_eq!(adjacent_vertices[1].0.id, 3);
        assert_eq!(adjacent_vertices[1].1, &20);

        let adjacent_vertices = graph
            .get_adjacent_vertices_with_edges(&2)
            .collect::<Vec<_>>();
        assert_eq!(adjacent_vertices.len(), 0);

        assert_eq!(
            graph
                .get_adjacent_vertices_with_edges(&4)
                .collect::<Vec<_>>()
                .len(),
            0
        );
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct MockWeightedEdge {
        weight: u32,
    }

    impl WeightedEdge for MockWeightedEdge {
        type WeightType = u32;

        fn get_weight(&self) -> Self::WeightType {
            self.weight
        }
    }

    #[test]
    fn test_get_total_weight_directed() {
        let mut graph: AdjacencyListGraph<u32, MockVertex, MockWeightedEdge> =
            AdjacencyListGraph::new(true);
        let vertex1 = MockVertex { id: 1 };
        let vertex2 = MockVertex { id: 2 };
        let vertex3 = MockVertex { id: 3 };

        graph.push_vertex(vertex1).unwrap();
        graph.push_vertex(vertex2).unwrap();
        graph.push_vertex(vertex3).unwrap();

        graph
            .push_edge(1, 2, MockWeightedEdge { weight: 10 })
            .unwrap();
        graph
            .push_edge(1, 3, MockWeightedEdge { weight: 20 })
            .unwrap();

        let total_weight = graph.get_total_weight();
        assert_eq!(total_weight, 30);
    }

    #[test]
    fn test_get_total_weight_undirected() {
        let mut graph: AdjacencyListGraph<u32, MockVertex, MockWeightedEdge> =
            AdjacencyListGraph::new(false);
        let vertex1 = MockVertex { id: 1 };
        let vertex2 = MockVertex { id: 2 };
        let vertex3 = MockVertex { id: 3 };

        graph.push_vertex(vertex1).unwrap();
        graph.push_vertex(vertex2).unwrap();
        graph.push_vertex(vertex3).unwrap();

        graph
            .push_edge(1, 2, MockWeightedEdge { weight: 10 })
            .unwrap();
        graph
            .push_edge(1, 3, MockWeightedEdge { weight: 20 })
            .unwrap();

        let total_weight = graph.get_total_weight();
        assert_eq!(total_weight, 30);
    }

    #[test]
    fn test_get_all_edges_directed() {
        let mut graph: AdjacencyListGraph<u32, MockVertex, u32> = AdjacencyListGraph::new(true);
        let v1 = MockVertex { id: 1 };
        let v2 = MockVertex { id: 2 };
        let v3 = MockVertex { id: 3 };
        graph.push_vertex(v1).unwrap();
        graph.push_vertex(v2).unwrap();
        graph.push_vertex(v3).unwrap();
        graph.push_edge(1, 2, 10).unwrap();
        graph.push_edge(2, 3, 20).unwrap();
        let mut edges = graph.get_all_edges().collect::<Vec<_>>();
        edges.sort_by_key(|(from, to, _)| (**from, **to));
        assert_eq!(edges, vec![(&1, &2, &10), (&2, &3, &20)]);
    }

    #[test]
    fn test_get_all_edges_undirected() {
        let mut graph: AdjacencyListGraph<u32, MockVertex, u32> = AdjacencyListGraph::new(false);
        let v1 = MockVertex { id: 1 };
        let v2 = MockVertex { id: 2 };
        let v3 = MockVertex { id: 3 };
        graph.push_vertex(v1).unwrap();
        graph.push_vertex(v2).unwrap();
        graph.push_vertex(v3).unwrap();
        graph.push_edge(1, 2, 10).unwrap();
        graph.push_edge(2, 3, 20).unwrap();
        let mut edges = graph.get_all_edges().collect::<Vec<_>>();
        edges.sort_by_key(|(from, to, _)| (**from, **to));
        // Only one direction per edge
        assert_eq!(edges, vec![(&1, &2, &10), (&2, &3, &20)]);
    }

    #[test]
    fn test_vertex_count() {
        let mut graph: AdjacencyListGraph<u32, MockVertex, ()> = AdjacencyListGraph::new(true);
        assert_eq!(graph.vertex_count(), 0);

        graph.push_vertex(MockVertex { id: 1 }).unwrap();
        assert_eq!(graph.vertex_count(), 1);

        graph.push_vertex(MockVertex { id: 2 }).unwrap();
        assert_eq!(graph.vertex_count(), 2);

        // Duplicate vertex should not increase count
        assert!(graph.push_vertex(MockVertex { id: 1 }).is_err());
        assert_eq!(graph.vertex_count(), 2);
    }

    #[test]
    fn test_edge_count_directed() {
        let mut graph: AdjacencyListGraph<u32, MockVertex, u32> = AdjacencyListGraph::new(true);
        graph.push_vertex(MockVertex { id: 1 }).unwrap();
        graph.push_vertex(MockVertex { id: 2 }).unwrap();
        graph.push_vertex(MockVertex { id: 3 }).unwrap();

        assert_eq!(graph.edge_count(), 0);

        graph.push_edge(1, 2, 10).unwrap();
        assert_eq!(graph.edge_count(), 1);

        graph.push_edge(2, 3, 20).unwrap();
        assert_eq!(graph.edge_count(), 2);

        // Duplicate edge should not increase count
        assert!(graph.push_edge(1, 2, 30).is_err());
        assert_eq!(graph.edge_count(), 2);
    }

    #[test]
    fn test_edge_count_undirected() {
        let mut graph: AdjacencyListGraph<u32, MockVertex, u32> = AdjacencyListGraph::new(false);
        graph.push_vertex(MockVertex { id: 1 }).unwrap();
        graph.push_vertex(MockVertex { id: 2 }).unwrap();
        graph.push_vertex(MockVertex { id: 3 }).unwrap();

        assert_eq!(graph.edge_count(), 0);

        graph.push_edge(1, 2, 10).unwrap();
        assert_eq!(graph.edge_count(), 1);

        graph.push_edge(2, 3, 20).unwrap();
        assert_eq!(graph.edge_count(), 2);

        // Duplicate edge should not increase count
        assert!(graph.push_edge(1, 2, 30).is_err());
        assert_eq!(graph.edge_count(), 2);
    }
}
