use std::fmt::Debug;

use crate::{
    graph::{
        adjacency_list::AdjacencyListGraph,
        traits::{GraphBase, WeightedEdge, WithID},
    },
    GraphError,
};
use delegate::delegate;

use super::adjacency_matrix::AdjacencyMatrixGraph;

#[derive(Debug)]
pub struct Graph<Backend> {
    backend: Backend,
}

// Public types for simplicity
pub type ListGraph<Vertex, Edge, Dir> = Graph<AdjacencyListGraph<Vertex, Edge, Dir>>;
pub type ListGraphBackend<Vertex, Edge, Dir> = AdjacencyListGraph<Vertex, Edge, Dir>;

pub type MatrixGraph<Vertex, Edge, Dir> = Graph<AdjacencyMatrixGraph<Vertex, Edge, Dir>>;
pub type MatrixGraphBackend<Vertex, Edge, Dir> = AdjacencyMatrixGraph<Vertex, Edge, Dir>;

impl<Backend> GraphBase for Graph<Backend>
where
    Backend: GraphBase,
{
    type Vertex = Backend::Vertex;
    type Edge = Backend::Edge;
    type Direction = Backend::Direction;

    fn new() -> Self
    where
        Self: Sized,
    {
        Graph {
            backend: Backend::new(),
        }
    }

    fn new_with_size(n_vertices: usize) -> Self
    where
        Self: Sized,
    {
        Graph {
            backend: Backend::new_with_size(n_vertices),
        }
    }

    fn from_vertices_and_edges(
        vertices: Vec<Self::Vertex>,
        edges: Vec<(
            <Self::Vertex as WithID>::IDType,
            <Self::Vertex as WithID>::IDType,
            Self::Edge,
        )>,
    ) -> Result<Self, GraphError<<Self::Vertex as WithID>::IDType>>
    where
        Self: Sized,
    {
        Backend::from_vertices_and_edges(vertices, edges).map(|backend| Graph { backend })
    }

    delegate!(
        to self.backend {
            fn push_vertex(
                &mut self,
                vertex: Self::Vertex,
            ) -> Result<(), GraphError<<Self::Vertex as WithID>::IDType>>;

            fn push_edge(
                &mut self,
                from: <Self::Vertex as WithID>::IDType,
                to: <Self::Vertex as WithID>::IDType,
                edge: Self::Edge,
            ) -> Result<(), GraphError<<Self::Vertex as WithID>::IDType>>;

            fn is_directed(&self) -> bool;

            fn get_vertex_by_id(&self, vertex_id: <Self::Vertex as WithID>::IDType) -> Option<&Self::Vertex>;

            fn get_vertex_by_id_mut(
                &mut self,
                vertex_id: <Self::Vertex as WithID>::IDType,
            ) -> Option<&mut Self::Vertex>;

            fn get_all_vertices<'a>(&'a self) -> impl Iterator<Item = &'a Self::Vertex>
            where
                Self::Vertex: 'a;

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

            fn get_adjacent_vertices<'a>(
                &'a self,
                vertex_id: <Self::Vertex as WithID>::IDType,
            ) -> impl Iterator<Item = &'a Self::Vertex>
            where
                Self::Vertex: 'a;

            fn get_adjacent_vertices_with_edges<'a>(
                &'a self,
                vertex_id: <Self::Vertex as WithID>::IDType,
            ) -> impl Iterator<Item = (&'a Self::Vertex, &'a Self::Edge)>
            where
                Self::Vertex: 'a,
                Self::Edge: 'a;

            fn vertex_count(&self) -> usize;

            fn edge_count(&self) -> usize;

            fn get_total_weight(&self) -> <Self::Edge as WeightedEdge>::WeightType
            where
                Self::Edge: WeightedEdge;
        }
    );
}

impl<Backend> Default for Graph<Backend>
where
    Backend: GraphBase,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::{Directed, Undirected};

    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct MockVertex {
        id: usize,
    }

    impl WithID for MockVertex {
        type IDType = usize;

        fn get_id(&self) -> usize {
            self.id
        }
    }

    #[rstest]
    fn test_push_vertex(
        #[values(
            ListGraph::<MockVertex, (), Directed>::new(),
            MatrixGraph::<MockVertex, (), Directed>::new(),
        )]
        mut graph: impl GraphBase<Vertex = MockVertex, Edge = (), Direction = Directed>,
    ) {
        let vertex1 = MockVertex { id: 0 };
        let vertex2 = MockVertex { id: 1 };

        assert!(graph.push_vertex(vertex1).is_ok());
        assert!(graph.push_vertex(vertex2).is_ok());
        assert!(matches!(
            graph.push_vertex(MockVertex { id: 1 }),
            Err(GraphError::DuplicateVertex(1))
        )); // Duplicate
    }

    #[rstest]
    fn test_push_edge(
        #[values(
            ListGraph::<MockVertex, i32, Directed>::new(),
            MatrixGraph::<MockVertex, i32, Directed>::new(),
        )]
        mut graph: impl GraphBase<Vertex = MockVertex, Edge = i32, Direction = Directed>,
    ) {
        let vertex1 = MockVertex { id: 0 };
        let vertex2 = MockVertex { id: 1 };

        graph.push_vertex(vertex1).unwrap();
        graph.push_vertex(vertex2).unwrap();

        assert!(graph.push_edge(0, 1, 10).is_ok());
        assert!(graph.push_edge(1, 0, 30).is_ok());

        assert!(matches!(
            graph.push_edge(0, 1, 20),
            Err(GraphError::DuplicateEdge(0, 1))
        )); // Duplicate edge
        assert!(matches!(
            graph.push_edge(2, 0, 40),
            Err(GraphError::VertexNotFound(2))
        )); // Non existent vertex
        assert!(matches!(
            graph.push_edge(0, 2, 40),
            Err(GraphError::VertexNotFound(2))
        )); // Non existent vertex
    }

    #[rstest]
    fn test_push_undirected_edge(
        #[values(
            ListGraph::<MockVertex, i32, Undirected>::new(),
            MatrixGraph::<MockVertex, i32, Undirected>::new()
        )]
        mut graph: impl GraphBase<Vertex = MockVertex, Edge = i32, Direction = Undirected>,
    ) {
        let vertex1 = MockVertex { id: 0 };
        let vertex2 = MockVertex { id: 1 };

        graph.push_vertex(vertex1).unwrap();
        graph.push_vertex(vertex2).unwrap();

        assert!(graph.push_edge(0, 1, 10).is_ok());
        assert!(matches!(
            graph.push_edge(0, 1, 20),
            Err(GraphError::DuplicateEdge(0, 1))
        )); // Duplicate edge

        let adj_0: Vec<_> = graph.get_adjacent_vertices_with_edges(0).collect();
        assert_eq!(adj_0.len(), 1);
        assert_eq!(adj_0[0].0.id, 1);
        assert_eq!(adj_0[0].1, &10);

        let adj_1: Vec<_> = graph.get_adjacent_vertices_with_edges(1).collect();
        assert_eq!(adj_1.len(), 1);
        assert_eq!(adj_1[0].0.id, 0);
        assert_eq!(adj_1[0].1, &10);
    }

    #[rstest]
    fn test_get_vertex(
        #[values(
            ListGraph::<MockVertex, (), Directed>::new(),
            MatrixGraph::<MockVertex, (), Directed>::new(),
        )]
        mut graph: impl GraphBase<Vertex = MockVertex, Edge = (), Direction = Directed>,
    ) {
        let vertex1 = MockVertex { id: 0 };
        let vertex2 = MockVertex { id: 1 };

        graph.push_vertex(vertex1).unwrap();
        graph.push_vertex(vertex2).unwrap();

        let v = graph.get_vertex_by_id(0).unwrap();
        assert_eq!(v.id, 0);
        let v = graph.get_vertex_by_id(1).unwrap();
        assert_eq!(v.id, 1);
        assert!(graph.get_vertex_by_id(2).is_none());
    }

    #[rstest]
    fn test_get_all_vertices(
        #[values(
            ListGraph::<MockVertex, (), Directed>::new(),
            MatrixGraph::<MockVertex, (), Directed>::new(),
        )]
        mut graph: impl GraphBase<Vertex = MockVertex, Edge = (), Direction = Directed>,
    ) {
        let vertex1 = MockVertex { id: 0 };
        let vertex2 = MockVertex { id: 1 };

        graph.push_vertex(vertex1).unwrap();
        graph.push_vertex(vertex2).unwrap();

        let vertices: Vec<_> = graph.get_all_vertices().map(|v| v.get_id()).collect();
        assert_eq!(vertices.len(), 2);
        assert!(vertices.contains(&0));
        assert!(vertices.contains(&1));
    }

    #[rstest]
    fn test_get_adjacent_vertices(
        #[values(
            ListGraph::<MockVertex, i32, Directed>::new(),
            MatrixGraph::<MockVertex, i32, Directed>::new(),
        )]
        mut graph: impl GraphBase<Vertex = MockVertex, Edge = i32, Direction = Directed>,
    ) {
        let vertex1 = MockVertex { id: 0 };
        let vertex2 = MockVertex { id: 1 };
        let vertex3 = MockVertex { id: 2 };

        graph.push_vertex(vertex1).unwrap();
        graph.push_vertex(vertex2).unwrap();
        graph.push_vertex(vertex3).unwrap();

        graph.push_edge(0, 1, 10).unwrap();
        graph.push_edge(0, 2, 20).unwrap();

        let adjacent_vertices = graph.get_adjacent_vertices(0).collect::<Vec<_>>();
        assert_eq!(adjacent_vertices.len(), 2);
        assert!(adjacent_vertices.iter().any(|v| v.id == 1));
        assert!(adjacent_vertices.iter().any(|v| v.id == 2));

        let adjacent_vertices = graph.get_adjacent_vertices(1).collect::<Vec<_>>();
        assert_eq!(adjacent_vertices.len(), 0);

        assert_eq!(graph.get_adjacent_vertices(3).collect::<Vec<_>>().len(), 0);
    }

    #[rstest]
    fn test_get_adjacent_vertices_with_edges(
        #[values(
            ListGraph::<MockVertex, i32, Directed>::new(),
            MatrixGraph::<MockVertex, i32, Directed>::new()
        )]
        mut graph: impl GraphBase<Vertex = MockVertex, Edge = i32, Direction = Directed>,
    ) {
        let vertex0 = MockVertex { id: 0 };
        let vertex1 = MockVertex { id: 1 };
        let vertex2 = MockVertex { id: 2 };

        graph.push_vertex(vertex0).unwrap();
        graph.push_vertex(vertex1).unwrap();
        graph.push_vertex(vertex2).unwrap();

        graph.push_edge(0, 1, 10).unwrap();
        graph.push_edge(0, 2, 20).unwrap();

        let adjacent_vertices = graph
            .get_adjacent_vertices_with_edges(0)
            .collect::<Vec<_>>();
        assert_eq!(adjacent_vertices.len(), 2);
        assert!(adjacent_vertices
            .iter()
            .any(|(v, e)| v.get_id() == 1 && **e == 10));
        assert!(adjacent_vertices
            .iter()
            .any(|(v, e)| v.get_id() == 2 && **e == 20));

        let adjacent_vertices = graph
            .get_adjacent_vertices_with_edges(1)
            .collect::<Vec<_>>();
        assert_eq!(adjacent_vertices.len(), 0);

        assert_eq!(
            graph
                .get_adjacent_vertices_with_edges(3)
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

    #[rstest]
    fn test_get_total_weight_directed(
        #[values(
            ListGraph::<MockVertex, MockWeightedEdge, Directed>::new(),
            MatrixGraph::<MockVertex, MockWeightedEdge, Directed>::new()
        )]
        mut graph: impl GraphBase<
            Vertex = MockVertex,
            Edge = MockWeightedEdge,
            Direction = Directed,
        >,
    ) {
        let vertex0 = MockVertex { id: 0 };
        let vertex1 = MockVertex { id: 1 };
        let vertex2 = MockVertex { id: 2 };

        graph.push_vertex(vertex0).unwrap();
        graph.push_vertex(vertex1).unwrap();
        graph.push_vertex(vertex2).unwrap();

        graph
            .push_edge(0, 1, MockWeightedEdge { weight: 10 })
            .unwrap();
        graph
            .push_edge(0, 2, MockWeightedEdge { weight: 20 })
            .unwrap();

        let total_weight = graph.get_total_weight();
        assert_eq!(total_weight, 30);
    }

    #[rstest]
    fn test_get_total_weight_undirected(
        #[values(
            ListGraph::<MockVertex, MockWeightedEdge, Undirected>::new(),
            MatrixGraph::<MockVertex, MockWeightedEdge, Undirected>::new()
        )]
        mut graph: impl GraphBase<
            Vertex = MockVertex,
            Edge = MockWeightedEdge,
            Direction = Undirected,
        >,
    ) {
        let vertex0 = MockVertex { id: 0 };
        let vertex1 = MockVertex { id: 1 };
        let vertex2 = MockVertex { id: 2 };

        graph.push_vertex(vertex0).unwrap();
        graph.push_vertex(vertex1).unwrap();
        graph.push_vertex(vertex2).unwrap();

        graph
            .push_edge(0, 1, MockWeightedEdge { weight: 10 })
            .unwrap();
        graph
            .push_edge(0, 2, MockWeightedEdge { weight: 20 })
            .unwrap();

        let total_weight = graph.get_total_weight();
        assert_eq!(total_weight, 30);
    }

    #[rstest]
    fn test_get_all_edges_directed(
        #[values(
            ListGraph::<MockVertex, i32, Directed>::new(),
            MatrixGraph::<MockVertex, i32, Directed>::new()
        )]
        mut graph: impl GraphBase<Vertex = MockVertex, Edge = i32, Direction = Directed>,
    ) {
        let v1 = MockVertex { id: 0 };
        let v2 = MockVertex { id: 1 };
        let v3 = MockVertex { id: 2 };
        graph.push_vertex(v1).unwrap();
        graph.push_vertex(v2).unwrap();
        graph.push_vertex(v3).unwrap();
        graph.push_edge(0, 1, 10).unwrap();
        graph.push_edge(1, 2, 20).unwrap();
        let mut edges = graph.get_all_edges().collect::<Vec<_>>();
        edges.sort_by_key(|(from, to, _)| (*from, *to));
        assert_eq!(edges, vec![(0, 1, &10), (1, 2, &20)]);
    }

    #[rstest]
    fn test_get_all_edges_undirected(
        #[values(
            ListGraph::<MockVertex, i32, Undirected>::new(),
            MatrixGraph::<MockVertex, i32, Undirected>::new()
        )]
        mut graph: impl GraphBase<Vertex = MockVertex, Edge = i32, Direction = Undirected>,
    ) {
        let v1 = MockVertex { id: 0 };
        let v2 = MockVertex { id: 1 };
        let v3 = MockVertex { id: 2 };
        graph.push_vertex(v1).unwrap();
        graph.push_vertex(v2).unwrap();
        graph.push_vertex(v3).unwrap();
        graph.push_edge(0, 1, 10).unwrap();
        graph.push_edge(1, 2, 20).unwrap();
        let mut edges = graph.get_all_edges().collect::<Vec<_>>();
        edges.sort_by_key(|(from, to, _)| (*from, *to));
        // Only one direction per edge
        assert_eq!(edges, vec![(0, 1, &10), (1, 2, &20)]);
    }

    #[rstest]
    fn test_vertex_count(
        #[values(
            ListGraph::<MockVertex, (), Directed>::new(),
            MatrixGraph::<MockVertex, (), Directed>::new()
        )]
        mut graph: impl GraphBase<Vertex = MockVertex, Edge = (), Direction = Directed>,
    ) {
        assert_eq!(graph.vertex_count(), 0);

        graph.push_vertex(MockVertex { id: 0 }).unwrap();
        assert_eq!(graph.vertex_count(), 1);

        graph.push_vertex(MockVertex { id: 1 }).unwrap();
        assert_eq!(graph.vertex_count(), 2);

        // Duplicate vertex should not increase count
        assert!(graph.push_vertex(MockVertex { id: 0 }).is_err());
        assert_eq!(graph.vertex_count(), 2);
    }

    #[rstest]
    fn test_edge_count_directed(
        #[values(
            ListGraph::<MockVertex, i32, Directed>::new(),
            MatrixGraph::<MockVertex, i32, Directed>::new()
        )]
        mut graph: impl GraphBase<Vertex = MockVertex, Edge = i32, Direction = Directed>,
    ) {
        graph.push_vertex(MockVertex { id: 0 }).unwrap();
        graph.push_vertex(MockVertex { id: 1 }).unwrap();
        graph.push_vertex(MockVertex { id: 2 }).unwrap();

        assert_eq!(graph.edge_count(), 0);

        graph.push_edge(0, 1, 10).unwrap();
        assert_eq!(graph.edge_count(), 1);

        graph.push_edge(1, 2, 20).unwrap();
        assert_eq!(graph.edge_count(), 2);

        // Duplicate edge should not increase count
        assert!(graph.push_edge(0, 1, 30).is_err());
        assert_eq!(graph.edge_count(), 2);
    }

    #[rstest]
    fn test_edge_count_undirected(
        #[values(
            ListGraph::<MockVertex, i32, Undirected>::new(),
            MatrixGraph::<MockVertex, i32, Undirected>::new()
        )]
        mut graph: impl GraphBase<Vertex = MockVertex, Edge = i32, Direction = Undirected>,
    ) {
        graph.push_vertex(MockVertex { id: 0 }).unwrap();
        graph.push_vertex(MockVertex { id: 1 }).unwrap();
        graph.push_vertex(MockVertex { id: 2 }).unwrap();

        assert_eq!(graph.edge_count(), 0);

        graph.push_edge(0, 1, 10).unwrap();
        assert_eq!(graph.edge_count(), 1);

        graph.push_edge(1, 2, 20).unwrap();
        assert_eq!(graph.edge_count(), 2);

        // Duplicate edge should not increase count
        assert!(graph.push_edge(0, 1, 30).is_err());
        assert_eq!(graph.edge_count(), 2);
    }
}
