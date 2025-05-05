use crate::{
    graph::{GraphBase, Path, WeightedEdge, WithID},
    Graph,
};

use super::TspResult;

impl<Backend> Graph<Backend>
where
    Backend: GraphBase,
    <Backend::Vertex as WithID>::IDType: Copy + PartialEq,
    Backend::Edge: WeightedEdge + Clone,
{
    /// Finds a path with the optimal TSP solution using a branch and bound brute force approach.
    ///
    /// # Requirements
    /// - `self` must be a fully connected graph with weights assigned to all edges.
    ///
    /// # Parameters
    /// - `start_vertex_id`: Optional ID of the vertex to start the TSP from. If `None`, a default starting vertex is chosen.
    ///
    /// # Returns
    /// - Returns a `TspResult<Backend>` containing the optimal path found, or an empty path if the graph is empty.
    ///
    /// # Panics
    /// - May panic if the graph is not fully connected.
    pub fn tsp_branch_and_bound(
        &self,
        start_vertex_id: Option<<Backend::Vertex as WithID>::IDType>,
    ) -> TspResult<Backend> {
        let (start_v, vertices) = match self.get_initial_vertex(start_vertex_id) {
            Some(v) => v,
            None => return Ok(Path::default()),
        };

        let mut remaining_vertices = vertices.collect::<Vec<_>>();
        todo!()
    }

    /// Recursive function to go through the different permutations
    fn branch_and_bound(
        &self,
        start_vertex_id: Option<<Backend::Vertex as WithID>::IDType>,
    ) -> TspResult<Backend> {
        let (start_v, vertices) = match self.get_initial_vertex(start_vertex_id) {
            Some(v) => v,
            None => return Ok(Path::default()),
        };

        let mut remaining_vertices = vertices.collect::<Vec<_>>();
        todo!()
    }
}
