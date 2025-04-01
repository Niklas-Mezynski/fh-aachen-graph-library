use std::{collections::VecDeque, fmt::Debug, hash::Hash};

use rustc_hash::FxHashSet;

use crate::{graph::WithID, Graph, GraphError};

impl<VId, Vertex, Edge> Graph<VId, Vertex, Edge>
where
    VId: Eq + Hash + Copy + Debug,
    Vertex: WithID<VId>,
    Edge: Clone,
{
    pub fn count_connected_subgraphs(&self) -> Result<u32, GraphError<VId>> {
        let mut vertices = VecDeque::from(self.get_all_vertices());
        let mut visited: FxHashSet<VId> = FxHashSet::default();

        let mut count: u32 = 0;

        'outer: while let Some(current_root) = vertices.pop_front() {
            let current_root_vid = current_root.get_id();
            if visited.contains(&current_root_vid) {
                continue;
            }
            for vertex in self.bfs_iter(current_root_vid)? {
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
}
