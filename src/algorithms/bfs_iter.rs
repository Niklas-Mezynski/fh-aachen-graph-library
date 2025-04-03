use std::{collections::VecDeque, fmt::Debug, hash::Hash};

use rustc_hash::FxHashSet;

use crate::{graph::WithID, Graph, GraphError};

pub struct BfsIter<'a, VId, Vertex, Edge>
where
    VId: Eq + Hash + Copy,
    Vertex: WithID<VId>,
    Edge:,
{
    graph: &'a Graph<VId, Vertex, Edge>,
    queue: VecDeque<VId>,
    visited: FxHashSet<VId>,
}

impl<'a, VId, Vertex, Edge> BfsIter<'a, VId, Vertex, Edge>
where
    VId: Eq + Hash + Copy,
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

        Ok(BfsIter {
            graph,
            queue,
            visited,
        })
    }
}

impl<'a, VId, Vertex, Edge> Iterator for BfsIter<'a, VId, Vertex, Edge>
where
    VId: Eq + Hash + Copy + Debug,
    Vertex: WithID<VId>,
    Edge: Clone,
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

pub struct BfsIterMut<'a, VId, Vertex, Edge>
where
    VId: Eq + Hash + Copy,
    Vertex: WithID<VId>,
    Edge:,
{
    graph: &'a mut Graph<VId, Vertex, Edge>,
    queue: VecDeque<VId>,
    visited: FxHashSet<VId>,
}

impl<'a, VId, Vertex, Edge> BfsIterMut<'a, VId, Vertex, Edge>
where
    VId: Eq + Hash + Copy,
    Vertex: WithID<VId>,
    Edge: Clone,
{
    fn new(
        graph: &'a mut Graph<VId, Vertex, Edge>,
        start_vertex: VId,
    ) -> Result<Self, GraphError<VId>> {
        let _ = graph.get_vertex_by_id(&start_vertex)?; // Check if it exists

        let queue = VecDeque::from([start_vertex]);

        let mut visited = FxHashSet::default();
        visited.insert(start_vertex);

        Ok(BfsIterMut {
            graph,
            queue,
            visited,
        })
    }
}

impl<'a, VId, Vertex, Edge> Iterator for BfsIterMut<'a, VId, Vertex, Edge>
where
    VId: Eq + Hash + Copy + Debug,
    Vertex: WithID<VId>,
    Edge: Clone,
{
    type Item = &'a mut Vertex;

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

            // Return mutable reference to the current vertex
            // This needs to use a method that returns &mut Vertex
            // SAFETY: This is safe because:
            // 1. We only return one mutable reference at a time
            // 2. Each vertex is visited exactly once (tracked by the visited set)
            // 3. The reference doesn't outlive the graph (tied to lifetime 'a)
            unsafe {
                let vertex_ptr = self.graph.get_vertex_by_id_mut(&next_id).expect(
                    "get_vertex_by_id_mut should not error as the vertices in the queue must exist",
                ) as *mut Vertex;

                Some(&mut *vertex_ptr)
            }
        } else {
            None
        }
    }
}

impl<VId, Vertex, Edge> Graph<VId, Vertex, Edge>
where
    VId: Eq + Hash + Copy,
    Vertex: WithID<VId>,
    Edge: Clone,
{
    pub fn bfs_iter(
        &self,
        start_vertex: VId,
    ) -> Result<BfsIter<VId, Vertex, Edge>, GraphError<VId>> {
        BfsIter::new(self, start_vertex)
    }

    pub fn bfs_iter_mut(
        &mut self,
        start_vertex: VId,
    ) -> Result<BfsIterMut<VId, Vertex, Edge>, GraphError<VId>> {
        BfsIterMut::new(self, start_vertex)
    }
}
