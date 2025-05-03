use std::marker::PhantomData;

use super::{
    error::GraphError, Directed, Direction, EdgeTuple, GraphBase, Undirected, WeightedEdge, WithID,
};

/// A graph data structure represented by an adjacency matrix.
///
/// # Type Parameters
/// - `Vertex`: The vertex type, which must implement [`WithID<VId>`].
/// - `Edge`: The edge type stored in the matrix. Can be any type, but typically represents edge weights or properties.
/// - `Dir`: Wether the graph is `Directed` or `Undirected`
///
/// # Important
/// **This struct assumes that vertex IDs are sequential and correspond to indices in the range `0..n`, where `n` is the number of vertices.**
/// If vertex IDs are not sequential or do not start at zero, the behavior is undefined and may result in panics or incorrect results.
///
/// # See Also
/// - [`Graph`]: The generic graph struct which contains detailed documentation for all public graph operations.
#[derive(Debug)]
pub struct AdjacencyMatrixGraph<Vertex: WithID, Edge, Dir: Direction> {
    vertices: Vec<Vertex>,
    matrix: Vec<Vec<Option<Edge>>>,
    _phantom: std::marker::PhantomData<Dir>,
}

impl<Vertex: WithID, Edge, Dir: Direction> AdjacencyMatrixGraph<Vertex, Edge, Dir>
where
    Vertex::IDType: Into<usize> + From<usize> + Copy,
{
    /// Create a new, empty Graph with an Adjacency List representation
    pub fn new() -> Self {
        AdjacencyMatrixGraph {
            vertices: vec![],
            matrix: vec![],
            _phantom: PhantomData,
        }
    }

    fn new_with_size(n_vertices: usize) -> Self
    where
        Edge: Clone,
    {
        AdjacencyMatrixGraph {
            vertices: Vec::with_capacity(n_vertices),
            matrix: vec![vec![None; n_vertices]; n_vertices],
            _phantom: PhantomData,
        }
    }

    fn push_edge_internal(
        &mut self,
        from: Vertex::IDType,
        to: Vertex::IDType,
        edge: Edge,
    ) -> Result<(), GraphError<Vertex::IDType>> {
        let from_idx: usize = from.into();
        let to_idx: usize = to.into();

        if self.vertices.get(from_idx).is_none() {
            return Err(GraphError::VertexNotFound(from));
        }

        if self.vertices.get(to_idx).is_none() {
            return Err(GraphError::VertexNotFound(to));
        }

        if self.matrix[from_idx][to_idx].is_some() {
            return Err(GraphError::DuplicateEdge(from, to));
        }

        self.matrix[from_idx][to_idx] = Some(edge);

        Ok(())
    }

    fn get_vertex_by_id_internal(&self, vertex_id: Vertex::IDType) -> Option<&Vertex> {
        self.vertices.get(vertex_id.into())
    }

    fn get_vertex_by_id_mut_internal(&mut self, vertex_id: Vertex::IDType) -> Option<&mut Vertex> {
        self.vertices.get_mut(vertex_id.into())
    }

    fn get_edge_internal(&self, from_id: Vertex::IDType, to_id: Vertex::IDType) -> Option<&Edge> {
        let from_idx: usize = from_id.into();
        let to_idx: usize = to_id.into();
        self.matrix[from_idx][to_idx].as_ref()
    }

    fn get_all_vertices_internal(&self) -> impl Iterator<Item = &Vertex> {
        self.vertices.iter()
    }

    fn get_adjacent_vertices_internal(
        &self,
        vertex_id: Vertex::IDType,
    ) -> Box<dyn Iterator<Item = &Vertex> + '_> {
        let vertex_idx = vertex_id.into();

        if vertex_idx >= self.vertices.len() {
            // Return an empty iterator of the correct type
            return Box::new(std::iter::empty());
        }

        Box::new(
            self.matrix[vertex_idx]
                .iter()
                .enumerate()
                .filter_map(move |(to_idx, edge)| {
                    if edge.is_some() {
                        self.vertices.get(to_idx)
                    } else {
                        None
                    }
                }),
        )
    }

    fn get_adjacent_vertices_with_edges_internal(
        &self,
        vertex_id: Vertex::IDType,
    ) -> Box<dyn Iterator<Item = (&Vertex, &Edge)> + '_> {
        let idx: usize = vertex_id.into();

        if idx >= self.vertices.len() {
            // Return an empty iterator of the correct type
            return Box::new(std::iter::empty());
        }

        Box::new(
            self.matrix[idx]
                .iter()
                .enumerate()
                .filter_map(move |(to_idx, edge)| {
                    edge.as_ref().map(|edge| (&self.vertices[to_idx], edge))
                }),
        )
    }

    fn vertex_count_internal(&self) -> usize {
        self.vertices.len()
    }

    fn push_vertex_internal(&mut self, vertex: Vertex) -> Result<(), GraphError<Vertex::IDType>>
    where
        Edge: Clone,
    {
        let expected_id = self.vertices.len();
        let idx: usize = vertex.get_id().into();
        if idx > expected_id {
            return Err(GraphError::OperationFailed(format!(
                "Vertex ID must be sequential in AdjacencyMatrixGraph and equal to the current number of vertices (expected {}, got {})",
                expected_id,
                vertex.get_id().into()
            )));
        }
        if idx < expected_id {
            return Err(GraphError::DuplicateVertex(idx.into()));
        }

        self.vertices.push(vertex);

        // Grow each existing row by one column (add None)
        for row in &mut self.matrix {
            row.push(None);
        }
        // Add a new row for the new vertex (all None)
        self.matrix.push(vec![None; self.vertices.len()]);

        Ok(())
    }
}

impl<Vertex: WithID, Edge, Dir: Direction> Default for AdjacencyMatrixGraph<Vertex, Edge, Dir>
where
    Vertex::IDType: Into<usize> + From<usize> + Copy,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Vertex, Edge> GraphBase for AdjacencyMatrixGraph<Vertex, Edge, Undirected>
where
    Vertex::IDType: Into<usize> + From<usize> + Copy,
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
        vertices: Vec<Self::Vertex>,
        edges: Vec<EdgeTuple<<Self::Vertex as WithID>::IDType, Self::Edge>>,
    ) -> Result<Self, GraphError<<Self::Vertex as WithID>::IDType>>
    where
        Self: Sized,
    {
        if vertices.is_empty() {
            return Ok(Self::new());
        }

        let n = vertices.len();
        let mut graph = Self::new_with_size(n);

        // Insert vertices
        for vertex in vertices {
            graph.vertices.push(vertex);
        }

        // Insert edges
        for (from, to, edge) in edges {
            let from_idx: usize = from.into();
            let to_idx: usize = to.into();

            graph.matrix[from_idx][to_idx] = Some(edge.clone());
            graph.matrix[to_idx][from_idx] = Some(edge);
        }

        Ok(graph)
    }

    fn push_vertex(
        &mut self,
        vertex: Self::Vertex,
    ) -> Result<(), GraphError<<Self::Vertex as WithID>::IDType>> {
        self.push_vertex_internal(vertex)
    }

    fn push_edge(
        &mut self,
        from: <Self::Vertex as WithID>::IDType,
        to: <Self::Vertex as WithID>::IDType,
        edge: Self::Edge,
    ) -> Result<(), GraphError<<Self::Vertex as WithID>::IDType>> {
        self.push_edge_internal(from, to, edge.clone())?;
        self.push_edge_internal(to, from, edge)?;
        Ok(())
    }

    fn is_directed(&self) -> bool {
        false
    }

    fn get_vertex_by_id(
        &self,
        vertex_id: <Self::Vertex as WithID>::IDType,
    ) -> Option<&Self::Vertex> {
        self.get_vertex_by_id_internal(vertex_id)
    }

    fn get_vertex_by_id_mut(
        &mut self,
        vertex_id: <Self::Vertex as WithID>::IDType,
    ) -> Option<&mut Self::Vertex> {
        self.get_vertex_by_id_mut_internal(vertex_id)
    }

    fn get_edge(
        &self,
        from_id: <Self::Vertex as WithID>::IDType,
        to_id: <Self::Vertex as WithID>::IDType,
    ) -> Option<&Self::Edge> {
        self.get_edge_internal(from_id, to_id)
    }

    fn get_all_vertices<'a>(&'a self) -> impl Iterator<Item = &'a Self::Vertex>
    where
        Self::Vertex: 'a,
    {
        self.get_all_vertices_internal()
    }

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
        Self::Edge: 'a,
    {
        self.matrix.iter().enumerate().flat_map(|(from, row)| {
            row.iter().enumerate().filter_map(move |(to, edge)| {
                if from <= to {
                    edge.as_ref().map(|edge| (from.into(), to.into(), edge))
                } else {
                    None
                }
            })
        })
    }

    fn get_adjacent_vertices<'a>(
        &'a self,
        vertex_id: <Self::Vertex as WithID>::IDType,
    ) -> impl Iterator<Item = &'a Self::Vertex>
    where
        Self::Vertex: 'a,
    {
        self.get_adjacent_vertices_internal(vertex_id)
    }

    fn get_adjacent_vertices_with_edges<'a>(
        &'a self,
        vertex_id: <Self::Vertex as WithID>::IDType,
    ) -> impl Iterator<Item = (&'a Self::Vertex, &'a Self::Edge)>
    where
        Self::Vertex: 'a,
        Self::Edge: 'a,
    {
        self.get_adjacent_vertices_with_edges_internal(vertex_id)
    }

    fn vertex_count(&self) -> usize {
        self.vertex_count_internal()
    }

    fn edge_count(&self) -> usize {
        let edge_count: usize = self
            .matrix
            .iter()
            .map(|row| row.iter().filter(|e| e.is_some()).count())
            .sum();

        edge_count / 2
    }

    fn get_total_weight(&self) -> <Self::Edge as WeightedEdge>::WeightType
    where
        Self::Edge: WeightedEdge,
    {
        let sum: Edge::WeightType = self
            .matrix
            .iter()
            .flat_map(|row| {
                row.iter()
                    .filter_map(move |edge| edge.as_ref().map(|edge| edge.get_weight()))
            })
            .sum();

        sum / 2.into()
    }
}

impl<Vertex, Edge> GraphBase for AdjacencyMatrixGraph<Vertex, Edge, Directed>
where
    Vertex::IDType: Into<usize> + From<usize> + Copy,
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
        vertices: Vec<Self::Vertex>,
        edges: Vec<EdgeTuple<<Self::Vertex as WithID>::IDType, Self::Edge>>,
    ) -> Result<Self, GraphError<<Self::Vertex as WithID>::IDType>>
    where
        Self: Sized,
    {
        if vertices.is_empty() {
            return Ok(Self::new());
        }

        let n = vertices.len();
        let mut graph = Self::new_with_size(n);

        // Insert vertices
        for vertex in vertices {
            graph.vertices.push(vertex);
        }

        // Insert edges
        for (from, to, edge) in edges {
            let from_idx: usize = from.into();
            let to_idx: usize = to.into();

            graph.matrix[from_idx][to_idx] = Some(edge);
        }

        Ok(graph)
    }

    fn push_vertex(
        &mut self,
        vertex: Self::Vertex,
    ) -> Result<(), GraphError<<Self::Vertex as WithID>::IDType>> {
        self.push_vertex_internal(vertex)
    }

    fn push_edge(
        &mut self,
        from: <Self::Vertex as WithID>::IDType,
        to: <Self::Vertex as WithID>::IDType,
        edge: Self::Edge,
    ) -> Result<(), GraphError<<Self::Vertex as WithID>::IDType>> {
        self.push_edge_internal(from, to, edge)?;
        Ok(())
    }

    fn is_directed(&self) -> bool {
        false
    }

    fn get_vertex_by_id(
        &self,
        vertex_id: <Self::Vertex as WithID>::IDType,
    ) -> Option<&Self::Vertex> {
        self.get_vertex_by_id_internal(vertex_id)
    }

    fn get_vertex_by_id_mut(
        &mut self,
        vertex_id: <Self::Vertex as WithID>::IDType,
    ) -> Option<&mut Self::Vertex> {
        self.get_vertex_by_id_mut_internal(vertex_id)
    }

    fn get_edge(
        &self,
        from_id: <Self::Vertex as WithID>::IDType,
        to_id: <Self::Vertex as WithID>::IDType,
    ) -> Option<&Self::Edge> {
        self.get_edge_internal(from_id, to_id)
    }

    fn get_all_vertices<'a>(&'a self) -> impl Iterator<Item = &'a Self::Vertex>
    where
        Self::Vertex: 'a,
    {
        self.get_all_vertices_internal()
    }

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
        Self::Edge: 'a,
    {
        self.matrix.iter().enumerate().flat_map(|(from, row)| {
            row.iter().enumerate().filter_map(move |(to, edge)| {
                edge.as_ref().map(|edge| (from.into(), to.into(), edge))
            })
        })
    }

    fn get_adjacent_vertices<'a>(
        &'a self,
        vertex_id: <Self::Vertex as WithID>::IDType,
    ) -> impl Iterator<Item = &'a Self::Vertex>
    where
        Self::Vertex: 'a,
    {
        self.get_adjacent_vertices_internal(vertex_id)
    }

    fn get_adjacent_vertices_with_edges<'a>(
        &'a self,
        vertex_id: <Self::Vertex as WithID>::IDType,
    ) -> impl Iterator<Item = (&'a Self::Vertex, &'a Self::Edge)>
    where
        Self::Vertex: 'a,
        Self::Edge: 'a,
    {
        self.get_adjacent_vertices_with_edges_internal(vertex_id)
    }

    fn vertex_count(&self) -> usize {
        self.vertex_count_internal()
    }

    fn edge_count(&self) -> usize {
        self.matrix
            .iter()
            .map(|row| row.iter().filter(|e| e.is_some()).count())
            .sum()
    }

    fn get_total_weight(&self) -> <Self::Edge as WeightedEdge>::WeightType
    where
        Self::Edge: WeightedEdge,
    {
        let sum: Edge::WeightType = self
            .matrix
            .iter()
            .flat_map(|row| {
                row.iter()
                    .filter_map(move |edge| edge.as_ref().map(|edge| edge.get_weight()))
            })
            .sum();

        sum
    }
}
