use std::hash::Hash;

use crate::{
    graph::{GraphBase, ListGraphBackend, Path, WeightedEdge, WithID},
    Graph,
};

use super::TspResult;

impl<Backend> Graph<Backend>
where
    Backend: GraphBase,
    <Backend::Vertex as WithID>::IDType: Copy + Eq + Hash,
    Backend::Vertex: Clone,
    Backend::Edge: WeightedEdge + Clone,
    ListGraphBackend<Backend::Vertex, Backend::Edge, Backend::Direction>:
        GraphBase<Vertex = Backend::Vertex, Edge = Backend::Edge, Direction = Backend::Direction>,
{
    /// Finds a path with a TSP solution using the double tree algorithm.
    /// It constructs an MST and runs a depth-first search on it to construct the Hamilton-tour.
    ///
    /// The solution is within twice the optimal solution cost.
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
    pub fn tsp_double_tree(
        &self,
        start_vertex_id: Option<<Backend::Vertex as WithID>::IDType>,
    ) -> TspResult<Backend> {
        let mut path = Path::default();

        // Get random start vertex
        let (start_v, _) = match self.get_initial_vertex(start_vertex_id) {
            Some(v) => v,
            None => return Ok(Path::default()),
        };

        // Generate MST
        let mst = self.mst_prim::<ListGraphBackend<_, _, _>>(Some(start_v))?;

        let mut prev_v = start_v;
        for current_v in mst.dfs_iter(start_v)?.skip(1).map(|v| v.get_id()) {
            path.push(
                prev_v,
                current_v,
                self.get_edge(prev_v, current_v)
                    .expect("Edge must exist as TSP works on complete graphs")
                    .to_owned(),
            );

            prev_v = current_v;
        }

        // Return to start_v
        path.push(
            prev_v,
            start_v,
            self.get_edge(prev_v, start_v)
                .expect("Edge must exist as TSP works on complete graphs")
                .to_owned(),
        );

        Ok(path)
    }
}
