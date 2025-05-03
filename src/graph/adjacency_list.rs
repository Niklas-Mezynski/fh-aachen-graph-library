use std::{fmt::Debug, hash::Hash};

use rustc_hash::{FxBuildHasher, FxHashMap};

use super::{
    error::GraphError,
    traits::{GraphBase, WithID},
    Directed, Direction, Undirected, WeightedEdge,
};

#[derive(Debug)]
pub struct AdjacencyListGraph<Vertex: WithID, Edge, Dir: Direction> {
    vertices: FxHashMap<Vertex::IDType, Vertex>,
    adjacency: FxHashMap<Vertex::IDType, Vec<(Vertex::IDType, Edge)>>,
    _phantom: std::marker::PhantomData<Dir>,
}

impl<Vertex: WithID, Edge, Dir: Direction> AdjacencyListGraph<Vertex, Edge, Dir>
where
    Vertex::IDType: Eq + Hash + PartialOrd + Copy,
    Vertex: WithID,
    Edge: Clone,
{
    /// Create a new, empty Graph with an Adjacency List representation
    pub fn new() -> Self {
        AdjacencyListGraph {
            vertices: FxHashMap::default(),
            adjacency: FxHashMap::default(),
            _phantom: std::marker::PhantomData,
        }
    }

    fn new_with_size(n_vertices: usize) -> Self
    where
        Self: Sized,
    {
        AdjacencyListGraph {
            vertices: FxHashMap::with_capacity_and_hasher(n_vertices, FxBuildHasher),
            adjacency: FxHashMap::default(),
            _phantom: std::marker::PhantomData,
        }
    }

    fn push_edge_internal(
        &mut self,
        from: Vertex::IDType,
        to: Vertex::IDType,
        edge: Edge,
    ) -> Result<(), GraphError<Vertex::IDType>>
    where
        Vertex::IDType: Eq + Hash,
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

    fn push_vertex(&mut self, vertex: Vertex) -> Result<(), GraphError<Vertex::IDType>> {
        let vid = vertex.get_id();
        if self.vertices.contains_key(&vid) {
            return Err(GraphError::DuplicateVertex(vid));
        }

        self.vertices.insert(vid, vertex);
        Ok(())
    }

    fn get_vertex_by_id(&self, vertex_id: Vertex::IDType) -> Option<&Vertex> {
        self.vertices.get(&vertex_id)
    }

    fn get_vertex_by_id_mut(&mut self, vertex_id: Vertex::IDType) -> Option<&mut Vertex> {
        self.vertices.get_mut(&vertex_id)
    }

    fn get_edge_internal(&self, from_id: Vertex::IDType, to_id: Vertex::IDType) -> Option<&Edge> {
        self.adjacency
            .get(&from_id)
            .and_then(|edges| edges.iter().find(|(to, _)| to == &to_id))
            .map(|(_, edge)| edge)
    }

    fn get_all_vertices<'a>(&'a self) -> impl Iterator<Item = &'a Vertex>
    where
        Vertex: 'a,
    {
        self.vertices.values()
    }

    fn get_adjacent_vertices<'a>(
        &'a self,
        vertex_id: Vertex::IDType,
    ) -> impl Iterator<Item = &'a Vertex>
    where
        Vertex: 'a,
    {
        self.adjacency
            .get(&vertex_id)
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
        vertex_id: Vertex::IDType,
    ) -> impl Iterator<Item = (&'a Vertex, &'a Edge)>
    where
        Vertex: 'a,
        Edge: 'a,
    {
        self.adjacency
            .get(&vertex_id)
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

    fn vertex_count(&self) -> usize {
        self.vertices.len()
    }
}

impl<Vertex: WithID, Edge, Dir: Direction> Default for AdjacencyListGraph<Vertex, Edge, Dir>
where
    Vertex::IDType: Eq + Hash + PartialOrd + Copy,
    Vertex: WithID,
    Edge: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Vertex, Edge> GraphBase for AdjacencyListGraph<Vertex, Edge, Directed>
where
    Vertex::IDType: Eq + Hash + PartialOrd + Copy,
    Vertex: WithID,
    Edge: Clone,
{
    type Vertex = Vertex;
    type Edge = Edge;
    type Direction = Directed;

    fn new() -> Self
    where
        Self: Sized,
    {
        Self::new()
    }

    fn new_with_size(n_vertices: usize) -> Self
    where
        Self: Sized,
    {
        Self::new_with_size(n_vertices)
    }

    fn from_vertices_and_edges(
        vertices: Vec<Vertex>,
        edges: Vec<(<Vertex as WithID>::IDType, <Vertex as WithID>::IDType, Edge)>,
    ) -> Result<Self, GraphError<<Vertex as WithID>::IDType>>
    where
        Self: Sized,
    {
        let mut graph = Self::new_with_size(vertices.len());
        for vertex in vertices {
            graph.push_vertex(vertex)?;
        }
        for (from, to, edge) in edges {
            graph.push_edge(from, to, edge)?;
        }
        Ok(graph)
    }

    fn push_vertex(&mut self, vertex: Vertex) -> Result<(), GraphError<Vertex::IDType>> {
        self.push_vertex(vertex)
    }

    fn push_edge(
        &mut self,
        from: Vertex::IDType,
        to: Vertex::IDType,
        edge: Edge,
    ) -> Result<(), GraphError<Vertex::IDType>> {
        self.push_edge_internal(from, to, edge)?;
        Ok(())
    }

    fn is_directed(&self) -> bool {
        true
    }

    fn get_vertex_by_id(&self, vertex_id: Vertex::IDType) -> Option<&Vertex> {
        self.get_vertex_by_id(vertex_id)
    }

    fn get_vertex_by_id_mut(&mut self, vertex_id: Vertex::IDType) -> Option<&mut Vertex> {
        self.get_vertex_by_id_mut(vertex_id)
    }

    fn get_edge(
        &self,
        from_id: <Self::Vertex as WithID>::IDType,
        to_id: <Self::Vertex as WithID>::IDType,
    ) -> Option<&Self::Edge> {
        self.get_edge_internal(from_id, to_id)
    }

    fn get_all_vertices<'a>(&'a self) -> impl Iterator<Item = &'a Vertex>
    where
        Vertex: 'a,
    {
        self.get_all_vertices()
    }

    fn get_adjacent_vertices<'a>(
        &'a self,
        vertex_id: Vertex::IDType,
    ) -> impl Iterator<Item = &'a Vertex>
    where
        Vertex: 'a,
    {
        self.get_adjacent_vertices(vertex_id)
    }

    fn get_adjacent_vertices_with_edges<'a>(
        &'a self,
        vertex_id: Vertex::IDType,
    ) -> impl Iterator<Item = (&'a Vertex, &'a Edge)>
    where
        Vertex: 'a,
        Edge: 'a,
    {
        self.get_adjacent_vertices_with_edges(vertex_id)
    }

    fn get_all_edges<'a>(
        &'a self,
    ) -> impl Iterator<Item = (Vertex::IDType, Vertex::IDType, &'a Edge)>
    where
        Edge: 'a,
    {
        self.adjacency.iter().flat_map(|(from_id, adjacency_list)| {
            adjacency_list
                .iter()
                .map(move |(to_id, edge)| (*from_id, *to_id, edge))
        })
    }

    fn vertex_count(&self) -> usize {
        self.vertex_count()
    }

    fn edge_count(&self) -> usize {
        let edge_count: usize = self.adjacency.values().map(|adj| adj.len()).sum();
        edge_count
    }

    fn get_total_weight(&self) -> <Edge>::WeightType
    where
        Edge: WeightedEdge,
    {
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

        sum
    }
}

impl<Vertex, Edge> GraphBase for AdjacencyListGraph<Vertex, Edge, Undirected>
where
    Vertex::IDType: Eq + Hash + PartialOrd + Copy,
    Vertex: WithID,
    Edge: Clone,
{
    type Vertex = Vertex;
    type Edge = Edge;
    type Direction = Undirected;

    fn new() -> Self
    where
        Self: Sized,
    {
        Self::new()
    }

    fn new_with_size(n_vertices: usize) -> Self
    where
        Self: Sized,
    {
        Self::new_with_size(n_vertices)
    }

    fn from_vertices_and_edges(
        vertices: Vec<Vertex>,
        edges: Vec<(<Vertex as WithID>::IDType, <Vertex as WithID>::IDType, Edge)>,
    ) -> Result<Self, GraphError<<Vertex as WithID>::IDType>>
    where
        Self: Sized,
    {
        let mut graph = Self::new_with_size(vertices.len());
        for vertex in vertices {
            graph.push_vertex(vertex)?;
        }
        for (from, to, edge) in edges {
            graph.push_edge(from, to, edge)?;
        }
        Ok(graph)
    }

    fn push_vertex(&mut self, vertex: Vertex) -> Result<(), GraphError<Vertex::IDType>> {
        self.push_vertex(vertex)
    }

    fn push_edge(
        &mut self,
        from: Vertex::IDType,
        to: Vertex::IDType,
        edge: Edge,
    ) -> Result<(), GraphError<Vertex::IDType>> {
        self.push_edge_internal(from, to, edge.clone())?;
        self.push_edge_internal(to, from, edge)?;
        Ok(())
    }

    fn is_directed(&self) -> bool {
        false
    }

    fn get_vertex_by_id(&self, vertex_id: Vertex::IDType) -> Option<&Vertex> {
        self.get_vertex_by_id(vertex_id)
    }

    fn get_vertex_by_id_mut(&mut self, vertex_id: Vertex::IDType) -> Option<&mut Vertex> {
        self.get_vertex_by_id_mut(vertex_id)
    }

    fn get_edge(
        &self,
        from_id: <Self::Vertex as WithID>::IDType,
        to_id: <Self::Vertex as WithID>::IDType,
    ) -> Option<&Self::Edge> {
        self.get_edge_internal(from_id, to_id)
    }

    fn get_all_vertices<'a>(&'a self) -> impl Iterator<Item = &'a Vertex>
    where
        Vertex: 'a,
    {
        self.get_all_vertices()
    }

    fn get_adjacent_vertices<'a>(
        &'a self,
        vertex_id: Vertex::IDType,
    ) -> impl Iterator<Item = &'a Vertex>
    where
        Vertex: 'a,
    {
        self.get_adjacent_vertices(vertex_id)
    }

    fn get_adjacent_vertices_with_edges<'a>(
        &'a self,
        vertex_id: Vertex::IDType,
    ) -> impl Iterator<Item = (&'a Vertex, &'a Edge)>
    where
        Vertex: 'a,
        Edge: 'a,
    {
        self.get_adjacent_vertices_with_edges(vertex_id)
    }

    fn get_all_edges<'a>(
        &'a self,
    ) -> impl Iterator<Item = (Vertex::IDType, Vertex::IDType, &'a Edge)>
    where
        Edge: 'a,
    {
        self.adjacency.iter().flat_map(|(from_id, adjacency_list)| {
            adjacency_list.iter().filter_map(move |(to_id, edge)| {
                if from_id <= to_id {
                    Some((*from_id, *to_id, edge))
                } else {
                    None
                }
            })
        })
    }

    fn vertex_count(&self) -> usize {
        self.vertex_count()
    }

    fn edge_count(&self) -> usize {
        let edge_count: usize = self.adjacency.values().map(|adj| adj.len()).sum();
        edge_count / 2
    }

    fn get_total_weight(&self) -> <Edge>::WeightType
    where
        Edge: WeightedEdge,
    {
        let sum: <Edge as WeightedEdge>::WeightType = self
            .adjacency
            .values()
            .map(|adjacency_list| {
                adjacency_list
                    .iter()
                    .map(|(_, edge)| edge.get_weight())
                    .sum()
            })
            .sum();

        sum / Edge::WeightType::from(2)
    }
}
