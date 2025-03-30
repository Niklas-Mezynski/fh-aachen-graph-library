use std::{collections::VecDeque, fmt::Debug, hash::Hash};

use rustc_hash::FxHashSet;

use crate::{graph::WithID, Graph, GraphError, GraphInterface};

impl<VId, Vertex: WithID<Vertex, VId>, Edge> Graph<VId, Vertex, Edge>
where
    VId: Debug + Eq + Hash + Copy,
    Vertex: Debug,
    Edge: Debug + Clone,
{
    pub fn count_connected_subgraphs(&self) -> Result<u32, GraphError<VId>> {
        let mut vertices = VecDeque::from(self.get_all_vertices());
        let mut visited: FxHashSet<VId> = FxHashSet::default();

        let mut count: u32 = 0;

        while let Some(current_root) = vertices.pop_front() {
            let current_root_vid = current_root.get_id();
            if visited.contains(&current_root_vid) {
                continue;
            }
            for vertex in self.bfs_iter(current_root_vid)? {
                // Remember that this vertex was already visited
                visited.insert(vertex.get_id());
            }

            // We traversed one whole graph, add one to the final count
            count += 1;
        }

        Ok(count)
    }
}
