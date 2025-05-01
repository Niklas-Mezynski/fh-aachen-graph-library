use std::{fmt::Debug, marker::PhantomData};

use crate::{
    graph::{
        adjacency_list::AdjacencyListGraph,
        traits::{GraphBase, WeightedEdge, WithID},
    },
    GraphError,
};
use delegate::delegate;

#[derive(Debug)]
pub struct Graph<Vertex, Edge, Dir, Backend> {
    backend: Backend,
    _phantom_vertex: PhantomData<Vertex>,
    _phantom_edge: PhantomData<Edge>,
    _phantom_dir: PhantomData<Dir>,
}

// Public types for simplicity
pub type ListGraph<Vertex, Edge, Dir> =
    Graph<Vertex, Edge, Dir, AdjacencyListGraph<Vertex, Edge, Dir>>;
pub type ListGraphBackend<Vertex, Edge, Dir> = AdjacencyListGraph<Vertex, Edge, Dir>;

impl<Vertex, Edge, Dir, Backend> GraphBase<Vertex, Edge, Dir> for Graph<Vertex, Edge, Dir, Backend>
where
    Vertex: WithID + Debug,
    Edge: Debug,
    Backend: GraphBase<Vertex, Edge, Dir>,
{
    type Direction = Dir;

    fn new() -> Self
    where
        Self: Sized,
    {
        Graph {
            backend: Backend::new(),
            _phantom_vertex: PhantomData,
            _phantom_edge: PhantomData,
            _phantom_dir: PhantomData,
        }
    }

    fn new_with_size(n_vertices: usize) -> Self
    where
        Self: Sized,
    {
        Graph {
            backend: Backend::new_with_size(n_vertices),
            _phantom_vertex: PhantomData,
            _phantom_edge: PhantomData,
            _phantom_dir: PhantomData,
        }
    }

    fn from_vertices_and_edges(
        vertices: Vec<Vertex>,
        edges: Vec<(<Vertex as WithID>::IDType, <Vertex as WithID>::IDType, Edge)>,
    ) -> Result<Self, GraphError<<Vertex as WithID>::IDType>>
    where
        Self: Sized,
    {
        Backend::from_vertices_and_edges(vertices, edges).map(|backend| Graph {
            backend,
            _phantom_vertex: PhantomData,
            _phantom_edge: PhantomData,
            _phantom_dir: PhantomData,
        })
    }

    delegate!(
        to self.backend {
            fn push_vertex(
            &mut self,
            vertex: Vertex,
        ) -> Result<(), GraphError<<Vertex as WithID>::IDType>>;

        fn push_edge(
            &mut self,
            from: <Vertex as WithID>::IDType,
            to: <Vertex as WithID>::IDType,
            edge: Edge,
        ) -> Result<(), GraphError<<Vertex as WithID>::IDType>>;

        fn is_directed(&self) -> bool;

        fn get_vertex_by_id(&self, vertex_id: <Vertex as WithID>::IDType) -> Option<&Vertex>;

        fn get_vertex_by_id_mut(
            &mut self,
            vertex_id: <Vertex as WithID>::IDType,
        ) -> Option<&mut Vertex>;

        fn get_all_vertices<'a>(&'a self) -> impl Iterator<Item = &'a Vertex>
        where
            Vertex: 'a;

        fn get_all_edges<'a>(
            &'a self,
        ) -> impl Iterator<
            Item = (
                <Vertex as WithID>::IDType,
                <Vertex as WithID>::IDType,
                &'a Edge,
            ),
        >
        where
            Edge: 'a;

        fn get_adjacent_vertices<'a>(
            &'a self,
            vertex_id: <Vertex as WithID>::IDType,
        ) -> impl Iterator<Item = &'a Vertex>
        where
            Vertex: 'a;

        fn get_adjacent_vertices_with_edges<'a>(
            &'a self,
            vertex_id: <Vertex as WithID>::IDType,
        ) -> impl Iterator<Item = (&'a Vertex, &'a Edge)>
        where
            Vertex: 'a,
            Edge: 'a;

        fn vertex_count(&self) -> usize;

        fn edge_count(&self) -> usize;

        fn get_total_weight(&self) -> <Edge>::WeightType
        where
            Edge: WeightedEdge;
        }
    );
}

impl<Vertex, Edge, Dir, Backend> Default for Graph<Vertex, Edge, Dir, Backend>
where
    Vertex: WithID + Debug,
    Edge: Debug,
    Backend: GraphBase<Vertex, Edge, Dir>,
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
        id: u32,
    }

    impl WithID for MockVertex {
        type IDType = u32;

        fn get_id(&self) -> u32 {
            self.id
        }
    }

    #[rstest]
    fn test_push_vertex(
        #[values(ListGraph::<MockVertex, (), Directed>::new())] mut graph: impl GraphBase<
            MockVertex,
            (),
            Directed,
        >,
    ) {
        let vertex1 = MockVertex { id: 1 };
        let vertex2 = MockVertex { id: 2 };

        assert!(graph.push_vertex(vertex1).is_ok());
        assert!(graph.push_vertex(vertex2).is_ok());
        assert!(matches!(
            graph.push_vertex(MockVertex { id: 1 }),
            Err(GraphError::DuplicateVertex(1))
        )); // Duplicate
    }

    #[rstest]
    fn test_push_edge(
        #[values(ListGraph::<MockVertex, i32, Directed>::new())] mut graph: impl GraphBase<
            MockVertex,
            i32,
            Directed,
        >,
    ) {
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

    #[rstest]
    fn test_push_undirected_edge(
        #[values(ListGraph::<MockVertex, i32, Undirected>::new())] mut graph: impl GraphBase<
            MockVertex,
            i32,
            Undirected,
        >,
    ) {
        let vertex1 = MockVertex { id: 1 };
        let vertex2 = MockVertex { id: 2 };

        graph.push_vertex(vertex1).unwrap();
        graph.push_vertex(vertex2).unwrap();

        assert!(graph.push_edge(1, 2, 10).is_ok());
        assert!(matches!(
            graph.push_edge(1, 2, 20),
            Err(GraphError::DuplicateEdge(1, 2))
        )); // Duplicate edge

        let adj_1: Vec<_> = graph.get_adjacent_vertices_with_edges(1).collect();
        assert_eq!(adj_1.len(), 1);
        assert_eq!(adj_1[0].0.id, 2);
        assert_eq!(adj_1[0].1, &10);

        let adj_2: Vec<_> = graph.get_adjacent_vertices_with_edges(2).collect();
        assert_eq!(adj_2.len(), 1);
        assert_eq!(adj_2[0].0.id, 1);
        assert_eq!(adj_2[0].1, &10);
    }

    #[rstest]
    fn test_get_vertex(
        #[values(ListGraph::<MockVertex, (), Directed>::new())] mut graph: impl GraphBase<
            MockVertex,
            (),
            Directed,
        >,
    ) {
        let vertex1 = MockVertex { id: 1 };
        let vertex2 = MockVertex { id: 2 };

        graph.push_vertex(vertex1).unwrap();
        graph.push_vertex(vertex2).unwrap();

        let v = graph.get_vertex_by_id(1).unwrap();
        assert_eq!(v.id, 1);
        let v = graph.get_vertex_by_id(2).unwrap();
        assert_eq!(v.id, 2);
        assert!(graph.get_vertex_by_id(3).is_none());
    }

    #[rstest]
    fn test_get_all_vertices(
        #[values(ListGraph::<MockVertex, (), Directed>::new())] mut graph: impl GraphBase<
            MockVertex,
            (),
            Directed,
        >,
    ) {
        let vertex1 = MockVertex { id: 1 };
        let vertex2 = MockVertex { id: 2 };

        graph.push_vertex(vertex1).unwrap();
        graph.push_vertex(vertex2).unwrap();

        let vertices: Vec<_> = graph.get_all_vertices().map(|v| v.get_id()).collect();
        assert_eq!(vertices.len(), 2);
        assert!(vertices.contains(&1));
        assert!(vertices.contains(&2));
    }

    #[rstest]
    fn test_get_adjacent_vertices(
        #[values(ListGraph::<MockVertex, i32, Directed>::new())] mut graph: impl GraphBase<
            MockVertex,
            i32,
            Directed,
        >,
    ) {
        let vertex1 = MockVertex { id: 1 };
        let vertex2 = MockVertex { id: 2 };
        let vertex3 = MockVertex { id: 3 };

        graph.push_vertex(vertex1).unwrap();
        graph.push_vertex(vertex2).unwrap();
        graph.push_vertex(vertex3).unwrap();

        graph.push_edge(1, 2, 10).unwrap();
        graph.push_edge(1, 3, 20).unwrap();

        let adjacent_vertices = graph.get_adjacent_vertices(1).collect::<Vec<_>>();
        assert_eq!(adjacent_vertices.len(), 2);
        assert!(adjacent_vertices.iter().any(|v| v.id == 2));
        assert!(adjacent_vertices.iter().any(|v| v.id == 3));

        let adjacent_vertices = graph.get_adjacent_vertices(2).collect::<Vec<_>>();
        assert_eq!(adjacent_vertices.len(), 0);

        assert_eq!(graph.get_adjacent_vertices(4).collect::<Vec<_>>().len(), 0);
    }

    #[rstest]
    fn test_get_adjacent_vertices_with_edges(
        #[values(ListGraph::<MockVertex, i32, Directed>::new())] mut graph: impl GraphBase<
            MockVertex,
            i32,
            Directed,
        >,
    ) {
        let vertex1 = MockVertex { id: 1 };
        let vertex2 = MockVertex { id: 2 };
        let vertex3 = MockVertex { id: 3 };

        graph.push_vertex(vertex1).unwrap();
        graph.push_vertex(vertex2).unwrap();
        graph.push_vertex(vertex3).unwrap();

        graph.push_edge(1, 2, 10).unwrap();
        graph.push_edge(1, 3, 20).unwrap();

        let adjacent_vertices = graph
            .get_adjacent_vertices_with_edges(1)
            .collect::<Vec<_>>();
        assert_eq!(adjacent_vertices.len(), 2);
        assert!(adjacent_vertices
            .iter()
            .any(|(v, e)| v.get_id() == 2 && **e == 10));
        assert!(adjacent_vertices
            .iter()
            .any(|(v, e)| v.get_id() == 3 && **e == 20));

        let adjacent_vertices = graph
            .get_adjacent_vertices_with_edges(2)
            .collect::<Vec<_>>();
        assert_eq!(adjacent_vertices.len(), 0);

        assert_eq!(
            graph
                .get_adjacent_vertices_with_edges(4)
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
        #[values(ListGraph::<MockVertex, MockWeightedEdge, Directed>::new())] mut graph: impl GraphBase<
            MockVertex,
            MockWeightedEdge,
            Directed,
        >,
    ) {
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

    #[rstest]
    fn test_get_total_weight_undirected(
        #[values(ListGraph::<MockVertex, MockWeightedEdge, Undirected>::new())]
        mut graph: impl GraphBase<MockVertex, MockWeightedEdge, Undirected>,
    ) {
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

    #[rstest]
    fn test_get_all_edges_directed(
        #[values(ListGraph::<MockVertex, i32, Directed>::new())] mut graph: impl GraphBase<
            MockVertex,
            i32,
            Directed,
        >,
    ) {
        let v1 = MockVertex { id: 1 };
        let v2 = MockVertex { id: 2 };
        let v3 = MockVertex { id: 3 };
        graph.push_vertex(v1).unwrap();
        graph.push_vertex(v2).unwrap();
        graph.push_vertex(v3).unwrap();
        graph.push_edge(1, 2, 10).unwrap();
        graph.push_edge(2, 3, 20).unwrap();
        let mut edges = graph.get_all_edges().collect::<Vec<_>>();
        edges.sort_by_key(|(from, to, _)| (*from, *to));
        assert_eq!(edges, vec![(1, 2, &10), (2, 3, &20)]);
    }

    #[rstest]
    fn test_get_all_edges_undirected(
        #[values(ListGraph::<MockVertex, i32, Undirected>::new())] mut graph: impl GraphBase<
            MockVertex,
            i32,
            Undirected,
        >,
    ) {
        let v1 = MockVertex { id: 1 };
        let v2 = MockVertex { id: 2 };
        let v3 = MockVertex { id: 3 };
        graph.push_vertex(v1).unwrap();
        graph.push_vertex(v2).unwrap();
        graph.push_vertex(v3).unwrap();
        graph.push_edge(1, 2, 10).unwrap();
        graph.push_edge(2, 3, 20).unwrap();
        let mut edges = graph.get_all_edges().collect::<Vec<_>>();
        edges.sort_by_key(|(from, to, _)| (*from, *to));
        // Only one direction per edge
        assert_eq!(edges, vec![(1, 2, &10), (2, 3, &20)]);
    }

    #[rstest]
    fn test_vertex_count(
        #[values(ListGraph::<MockVertex, (), Directed>::new())] mut graph: impl GraphBase<
            MockVertex,
            (),
            Directed,
        >,
    ) {
        assert_eq!(graph.vertex_count(), 0);

        graph.push_vertex(MockVertex { id: 1 }).unwrap();
        assert_eq!(graph.vertex_count(), 1);

        graph.push_vertex(MockVertex { id: 2 }).unwrap();
        assert_eq!(graph.vertex_count(), 2);

        // Duplicate vertex should not increase count
        assert!(graph.push_vertex(MockVertex { id: 1 }).is_err());
        assert_eq!(graph.vertex_count(), 2);
    }

    #[rstest]
    fn test_edge_count_directed(
        #[values(ListGraph::<MockVertex, i32, Directed>::new())] mut graph: impl GraphBase<
            MockVertex,
            i32,
            Directed,
        >,
    ) {
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

    #[rstest]
    fn test_edge_count_undirected(
        #[values(ListGraph::<MockVertex, i32, Undirected>::new())] mut graph: impl GraphBase<
            MockVertex,
            i32,
            Undirected,
        >,
    ) {
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
