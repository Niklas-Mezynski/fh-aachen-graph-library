use std::{collections::VecDeque, fmt::Debug, hash::Hash};

use rustc_hash::FxHashSet;

use crate::{Graph, WithID};

pub struct BfsIterator<'a, VId, Vertex, Edge>
where
    VId: 'static + Debug + Eq + Hash,
    Vertex: WithID<Vertex, VId> + 'static,
    Edge: 'static,
{
    graph: &'a Graph<VId, Vertex, Edge>,
    queue: VecDeque<VId>,
    visited: FxHashSet<VId>,
}

impl<'a, VId, Vertex, Edge> BfsIterator<'a, VId, Vertex, Edge>
where
    VId: 'static + Debug + Eq + Hash + Copy,
    Vertex: WithID<Vertex, VId> + 'static,
    Edge: 'static,
{
    pub fn new(graph: &'a Graph<VId, Vertex, Edge>, start_vertex: VId) -> Self {
        let queue = VecDeque::from([start_vertex]);
        let mut visited = FxHashSet::default();
        visited.insert(start_vertex);
        BfsIterator {
            graph,
            queue,
            visited,
        }
    }
}

impl<'a, VId, Vertex, Edge> Iterator for BfsIterator<'a, VId, Vertex, Edge>
where
    VId: 'static + Debug + Eq + Hash + Copy,
    Vertex: 'static + WithID<Vertex, VId> + Debug,
    Edge: 'static + Debug + Clone,
{
    type Item = &'a Vertex;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_id) = self.queue.pop_front() {
            // Add unvisited neighbors to queue
            let neighbors = self.graph.get_adjacent_vertices(next_id);
            // Return the current vertex

            todo!()
        } else {
            None
        }
    }
}
