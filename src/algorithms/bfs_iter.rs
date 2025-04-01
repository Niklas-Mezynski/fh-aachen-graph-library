use std::{collections::VecDeque, fmt::Debug, hash::Hash};

use rustc_hash::FxHashSet;

use crate::{graph::WithID, Graph, GraphError};

pub struct BfsIterator<'a, VId, Vertex, Edge>
where
    VId: Eq + Hash + Copy + 'static,
    Vertex: WithID<VId> + 'static,
    Edge: 'static,
{
    graph: &'a Graph<VId, Vertex, Edge>,
    queue: VecDeque<VId>,
    visited: FxHashSet<VId>,
}

impl<'a, VId, Vertex, Edge> BfsIterator<'a, VId, Vertex, Edge>
where
    VId: Eq + Hash + Copy + Debug,
    Vertex: WithID<VId>,
    Edge: Clone,
{
    fn new(
        graph: &'a Graph<VId, Vertex, Edge>,
        start_vertex: VId,
    ) -> Result<Self, GraphError<VId>> {
        let _ = graph.get_vertex_by_id(&start_vertex)?; // Check if it exists

        let queue = VecDeque::from([start_vertex]);

        let mut visited = FxHashSet::default();
        visited.insert(start_vertex);

        Ok(BfsIterator {
            graph,
            queue,
            visited,
        })
    }
}

impl<'a, VId, Vertex, Edge> Iterator for BfsIterator<'a, VId, Vertex, Edge>
where
    VId: Eq + Hash + Copy + Debug + 'static,
    Vertex: WithID<VId> + 'static,
    Edge: Clone + 'static,
{
    type Item = &'a Vertex;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_id) = self.queue.pop_front() {
            // Add unvisited neighbors to queue
            let neighbors = self.graph.get_adjacent_vertices(&next_id).expect(
                "get_adjacent_vertices should not error as the vertices in the queue must exist",
            );
            for v in neighbors {
                let vid = v.get_id();
                if !self.visited.contains(&vid) {
                    self.visited.insert(vid);
                    self.queue.push_back(vid);
                }
            }

            // Return the current vertex
            Some(self.graph.get_vertex_by_id(&next_id).expect(
                "get_vertex_by_id should not error as the vertices in the queue must exist",
            ))
        } else {
            None
        }
    }
}

impl<VId, Vertex, Edge> Graph<VId, Vertex, Edge>
where
    VId: Eq + Hash + Copy + Debug,
    Vertex: WithID<VId>,
    Edge: Clone,
{
    pub fn bfs_iter(
        &self,
        start_vertex: VId,
    ) -> Result<BfsIterator<VId, Vertex, Edge>, GraphError<VId>> {
        BfsIterator::new(self, start_vertex)
    }
}
