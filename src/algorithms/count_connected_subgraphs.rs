use std::{collections::VecDeque, fmt::Debug, hash::Hash};

use rustc_hash::FxHashSet;

use crate::{
    algorithms::iter::TraversalType,
    graph::{GraphBase, WithID},
    Graph, GraphError,
};

impl<Vertex, Edge, Dir, Backend> Graph<Vertex, Edge, Dir, Backend>
where
    Backend: GraphBase<Vertex, Edge, Dir>,
    Vertex: WithID + Clone + Debug,
    Vertex::IDType: Copy + Eq + Hash,
    Edge: Debug,
{
    /// Counts the number of connected subgraphs in the graph.
    ///
    /// Optionally accepts an iterator type to specify which traversal algorithm to use.
    pub fn count_connected_subgraphs(
        &self,
        traversal_type: Option<TraversalType>,
    ) -> Result<u32, GraphError<Vertex::IDType>> {
        let iter_type = traversal_type.unwrap_or_default(); // Default to BFS
        let mut vertices = self.get_all_vertices().collect::<VecDeque<_>>();
        let mut visited: FxHashSet<Vertex::IDType> = FxHashSet::default();

        let mut count: u32 = 0;

        'outer: while let Some(current_root) = vertices.pop_front() {
            let current_root_vid = current_root.get_id();
            if visited.contains(&current_root_vid) {
                continue;
            }

            for vertex in self.iter(current_root_vid, iter_type)? {
                let vid = vertex.get_id();

                // If this vertex has been visited already, we are traversing a subgraph that was already counted -> abort
                // (This may happen in directed graphs, if we start at a vertex that is not reachable by traversal)
                if visited.contains(&vid) {
                    continue 'outer;
                }

                // Remember that this vertex was already visited
                visited.insert(vid);
            }

            // We traversed one whole graph, add one to the final count
            count += 1;
        }

        Ok(count)
    }

    /// Method that uses BFS by default
    pub fn count_connected_subgraphs_with_default_traversal(
        &self,
    ) -> Result<u32, GraphError<Vertex::IDType>> {
        self.count_connected_subgraphs(None)
    }
}
