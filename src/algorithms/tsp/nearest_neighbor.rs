use std::hash::Hash;

use crate::{
    graph::{GraphBase, Path, WeightedEdge, WithID},
    Graph,
};

use super::TspResult;

impl<Backend> Graph<Backend>
where
    Backend: GraphBase,
    <Backend::Vertex as WithID>::IDType: Copy + Eq + Hash,
    Backend::Edge: WeightedEdge + Clone,
{
    /// Finds a path with a TSP solution using the nearest neighbor algorithm.
    ///
    /// There is no guarantee for the quality of the solution.
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
    pub fn tsp_nearest_neighbor(
        &self,
        start_vertex_id: Option<<Backend::Vertex as WithID>::IDType>,
    ) -> TspResult<Backend> {
        // Get random start vertex
        let (start_v, remaining) = match self.get_initial_vertex(start_vertex_id) {
            Some(v) => v,
            None => return Ok(Path::default()),
        };

        let mut current_path = vec![start_v];
        let mut remaining = remaining.collect::<Vec<_>>();

        while !remaining.is_empty() {
            let current = current_path.last().unwrap();

            let next_v_idx = remaining
                .iter()
                .enumerate()
                .map(|(i, v)| (i, self.get_edge(*current, *v).unwrap().get_weight()))
                .min_by(|(_to, edge), (_to_other, edge_other)| {
                    edge.partial_cmp(edge_other)
                        .expect("Graph weights must not contain NaN values")
                })
                .map(|(to, _edge)| to)
                .unwrap();

            // Add to path
            current_path.push(remaining[next_v_idx]);

            // Remove from the remaining vec
            let last_idx = remaining.len() - 1;
            remaining.swap(next_v_idx, last_idx);
            remaining.pop();
        }

        // Complete the cycle (back to start)
        current_path.push(start_v);

        // Construct the final path
        let mut path = Path::default();

        for window in current_path.windows(2) {
            let from_v = window[0];
            let to_v = window[1];
            let edge = self.get_edge(from_v, to_v).unwrap().clone();
            path.push(from_v, to_v, edge);
        }
        Ok(path)
    }
}
