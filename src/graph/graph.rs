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
