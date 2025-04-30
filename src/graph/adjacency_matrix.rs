// use super::{
//     error::GraphError,
//     traits::{GraphInterface, WithID},
//     WeightedEdge, WeightedGraphInterface,
// };

// /// A graph data structure represented by an adjacency matrix.
// ///
// /// # Type Parameters
// /// - `VId`: The type used for vertex IDs. Must be convertible to and from `usize`.
// /// - `Vertex`: The vertex type, which must implement [`WithID<VId>`].
// /// - `Edge`: The edge type stored in the matrix. Can be any type, but typically represents edge weights or properties.
// ///
// /// # Important
// /// **This struct assumes that vertex IDs are sequential and correspond to indices in the range `0..n`, where `n` is the number of vertices.**
// /// If vertex IDs are not sequential or do not start at zero, the behavior is undefined and may result in panics or incorrect results.
// ///
// /// # See Also
// /// - [`Graph`]: The generic graph struct which contains detailed documentation for all public graph operations.
// #[derive(Debug)]
// pub struct AdjacencyMatrixGraph<VId, Vertex: WithID<VId>, Edge> {
//     vertices: Vec<Vertex>,
//     matrix: Vec<Vec<Option<Edge>>>,
//     is_directed: bool,
//     _phantom: std::marker::PhantomData<VId>,
// }

// impl<VId, Vertex: WithID<VId>, Edge> AdjacencyMatrixGraph<VId, Vertex, Edge>
// where
//     VId: Into<usize> + From<usize>,
// {
//     /// Create a new, empty Graph with an Adjacency List representation
//     pub fn new(is_directed: bool) -> Self {
//         todo!()
//     }

//     /// Create a new Graph and tries to preallocate data structures based on the number of vertices/edges
//     pub fn new_with_size(
//         vertex_count: Option<usize>,
//         _edge_count: Option<usize>,
//         is_directed: bool,
//     ) -> Self
//     where
//         Self: Sized,
//     {
//         todo!()
//     }
// }

// impl<VId, Vertex, Edge> GraphInterface<VId, Vertex, Edge>
//     for AdjacencyMatrixGraph<VId, Vertex, Edge>
// where
//     VId: Into<usize> + From<usize> + Eq + PartialOrd + Copy,
//     Vertex: WithID<VId>,
//     Edge: Clone,
// {
//     fn push_vertex(&mut self, vertex: Vertex) -> Result<(), GraphError<VId>> {
//         todo!()
//     }

//     fn push_edge(&mut self, from: VId, to: VId, edge: Edge) -> Result<(), GraphError<VId>> {
//         todo!()
//     }

//     fn is_directed(&self) -> bool {
//         self.is_directed
//     }

//     fn get_vertex_by_id(&self, vertex_id: VId) -> Option<&Vertex> {
//         self.vertices.get(vertex_id.into())
//     }

//     fn get_vertex_by_id_mut(&mut self, vertex_id: VId) -> Option<&mut Vertex> {
//         self.vertices.get_mut(vertex_id.into())
//     }

//     fn get_all_vertices<'a>(&'a self) -> impl Iterator<Item = &'a Vertex>
//     where
//         Vertex: 'a,
//     {
//         self.vertices.iter()
//     }

//     fn get_all_edges<'a>(&'a self) -> impl Iterator<Item = (VId, VId, &'a Edge)>
//     where
//         VId: 'a,
//         Edge: 'a,
//     {
//         self.matrix.iter().enumerate().flat_map(|(from, row)| {
//             row.into_iter()
//                 .enumerate()
//                 .filter_map(move |(to, edge)| match edge {
//                     Some(edge) => Some((from.into(), to.into(), edge)),
//                     None => None,
//                 })
//         })
//     }

//     fn get_adjacent_vertices<'a>(&'a self, vertex_id: VId) -> impl Iterator<Item = &'a Vertex>
//     where
//         Vertex: 'a,
//     {
//         let idx = vertex_id.into();
//         self.matrix[idx]
//             .iter()
//             .enumerate()
//             .filter_map(move |(to_idx, edge)| {
//                 if edge.is_some() {
//                     self.vertices.get(to_idx)
//                 } else {
//                     None
//                 }
//             })
//     }

//     fn get_adjacent_vertices_with_edges<'a>(
//         &'a self,
//         vertex_id: VId,
//     ) -> impl Iterator<Item = (&'a Vertex, &'a Edge)>
//     where
//         Vertex: 'a,
//         Edge: 'a,
//     {
//         let idx: usize = vertex_id.into();
//         self.matrix[idx]
//             .iter()
//             .enumerate()
//             .filter_map(move |(to_idx, edge)| {
//                 edge.as_ref().map(|edge| (&self.vertices[to_idx], edge))
//             })
//     }

//     fn vertex_count(&self) -> usize {
//         self.vertices.len()
//     }

//     fn edge_count(&self) -> usize {
//         self.matrix
//             .iter()
//             .map(|row| row.iter().filter(|e| e.is_some()).count())
//             .sum()
//     }
// }

// impl<VId, Vertex, Edge> WeightedGraphInterface<VId, Vertex, Edge>
//     for AdjacencyMatrixGraph<VId, Vertex, Edge>
// where
//     VId: Into<usize> + From<usize> + Eq + PartialOrd + Copy,
//     Vertex: WithID<VId>,
//     Edge: WeightedEdge + Clone,
// {
//     fn get_total_weight(&self) -> Edge::WeightType {
//         todo!()
//     }
// }
